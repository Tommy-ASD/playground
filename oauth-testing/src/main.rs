//!
//! This example showcases the Github OAuth2 process for requesting access to the user's public repos and
//! email address.
//!
//! Before running it, you'll need to generate your own Github OAuth2 credentials.
//!
//! In order to run the example call:
//!
//! ```sh
//! GITHUB_CLIENT_ID=xxx GITHUB_CLIENT_SECRET=yyy cargo run --example github
//! ```
//!
//! ...and follow the instructions.
//!

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    reqwest::{async_http_client, Error as OAuthError},
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    RedirectUrl, RequestTokenError, RevocationErrorResponseType, Scope, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse,
    TokenResponse, TokenUrl,
};

use reqwest::Error as ReqwestError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use traceback_error::{traceback, TracebackError};

use std::env;
use tokio::net::TcpListener;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use url::Url;

use serde_json::json;

type ClientWithGenerics = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
>;

type TokenResult = Result<
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    RequestTokenError<OAuthError<ReqwestError>, StandardErrorResponse<BasicErrorResponseType>>,
>;
type Token = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

use dotenv_codegen::dotenv;

use crate::types::{DiscordIdentity, GithubIdentity, UserType, JWT};

const PRIVATE_KEY: &[u8] = include_bytes!("../private_key.pem");
const PUBLIC_KEY: &[u8] = include_bytes!("../public_key.pem");

pub mod types;

#[derive(Clone, Debug)]
pub struct OAuth2Info {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
}

impl OAuth2Info {
    fn github_default() -> Self {
        let client_id = dotenv!("GITHUB_CLIENT_ID");
        let client_secret = dotenv!("GITHUB_CLIENT_SECRET");
        let auth_url = "https://github.com/login/oauth/authorize".to_string();
        let token_url = "https://github.com/login/oauth/access_token".to_string();
        OAuth2Info {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            auth_url,
            token_url,
            redirect_url: "http://localhost:8080/oauth_callback".to_string(),
        }
    }
    fn discord_default() -> Self {
        let client_id = dotenv!("DISCORD_CLIENT_ID");
        let client_secret = dotenv!("DISCORD_CLIENT_SECRET");
        let auth_url = "https://discord.com/api/oauth2/authorize".to_string();
        let token_url = "https://discord.com/api/oauth2/token".to_string();

        OAuth2Info {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            auth_url,
            token_url,
            redirect_url: "http://localhost:49103/oauth/callback/discord".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum OAuthType {
    Standard,
    Github,
}

#[derive(Clone, Debug)]
struct OAuthParams {
    pub provider_name: String,
    pub general_info: OAuth2Info,
    pub scopes: Vec<Scope>,
    pub oauth_type: OAuthType,
}

impl OAuthParams {
    fn discord_default() -> Self {
        OAuthParams {
            provider_name: "Discord".to_string(),
            general_info: OAuth2Info::discord_default(),
            scopes: vec![Scope::new("identify".to_string())],
            oauth_type: OAuthType::Standard,
        }
    }
    fn github_default() -> Self {
        OAuthParams {
            provider_name: "Github".to_string(),
            general_info: OAuth2Info::github_default(),
            scopes: vec![],
            oauth_type: OAuthType::Github,
        }
    }
}

fn build_client(info: OAuth2Info) -> Result<ClientWithGenerics, TracebackError> {
    let redirect_url = match RedirectUrl::new(info.redirect_url.to_string()) {
        Ok(url) => url,
        Err(e) => {
            return Err(traceback!(err e, format!(
                "Invalid redirect URL: {}",
                info.redirect_url.to_string()
            )))
        }
    };
    let client = BasicClient::new(
        ClientId::new(info.client_id),
        Some(ClientSecret::new(info.client_secret)),
        AuthUrl::new(info.auth_url).expect("Invalid authorization endpoint URL"),
        Some(TokenUrl::new(info.token_url).expect("Invalid token endpoint URL")),
    )
    .set_redirect_uri(redirect_url);
    Ok(client)
}

#[traceback_derive::traceback]
async fn oauth(params: OAuthParams) -> Result<String, TracebackError> {
    let client = match build_client(params.general_info.clone()) {
        Ok(client) => client,
        Err(e) => return Err(traceback!(err e, "Failed to build client")),
    };

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's public repos and email.
        .add_scopes(params.scopes.iter().cloned())
        .url();

    println!(
        "\nOpen this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );
    Ok(tcp_server(client, csrf_state, params).await?)
}

#[traceback_derive::traceback]
async fn tcp_server(
    client: ClientWithGenerics,
    csrf_state: CsrfToken,
    params: OAuthParams,
) -> Result<String, TracebackError> {
    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            let (code, state) = parse_oauth_callback_request(&mut stream).await.unwrap();

            respond_to_oauth_callback(&mut stream).await.unwrap();

            // We now have the code, and can exchange it for a token.
            println!(
                "{provider_name} returned the following code:\n{}\n",
                code.secret(),
                provider_name = params.provider_name
            );
            println!(
                "{provider_name} returned the following state:\n{} (expected `{}`)\n",
                state.secret(),
                csrf_state.secret(),
                provider_name = params.provider_name
            );

            // Exchange the code with a token.
            let token_res: TokenResult = client
                .exchange_code(code)
                .request_async(async_http_client)
                .await;

            println!(
                "{provider_name} returned the following token:\n{token_res:?}\n",
                provider_name = params.provider_name
            );
            let token = match params.oauth_type {
                OAuthType::Standard => handle_standard_token(token_res)?,
                OAuthType::Github => handle_github_token(token_res)?,
            };

            // The server will terminate itself after collecting the first code.
            return Ok(token);
        }
    }
}

#[traceback_derive::traceback]
async fn parse_oauth_callback_request(
    stream: &mut TcpStream,
) -> Result<(AuthorizationCode, CsrfToken), TracebackError> {
    let mut reader = BufReader::new(stream);

    let mut request_line = String::new();
    // Read the first line of the stream.
    // Reads until a newline is encountered (which is discarded).
    // Result should be "GET /oauth_callback?code={code} HTTP/1.1\r\n".
    reader.read_line(&mut request_line).await.unwrap();

    println!("Line: {request_line}");

    // With that, we can get the 2nd element of the request_line split by whitespace.
    // It should be "/oauth_callback?code={code}".
    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
    // Then, we can create a `Url` from it, which will parse the query string.
    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

    // This iterates over the query string pairs, and finds the first one which has
    // "code" as the first element. Then it takes the second element of that pair,
    let code_pair = url
        .query_pairs()
        .find(|pair| {
            let &(ref key, _) = pair;
            key == "code"
        })
        .unwrap();

    // The formerly mentioned second element is the authorization code.
    let (_, value) = code_pair;
    let code = AuthorizationCode::new(value.into_owned());

    // Similar to above, but for the state.
    let state_pair = url
        .query_pairs()
        .find(|pair| {
            let &(ref key, _) = pair;
            key == "state"
        })
        .unwrap();

    let (_, value) = state_pair;
    let state = CsrfToken::new(value.into_owned());

    Ok((code, state))
}

#[traceback_derive::traceback]
async fn respond_to_oauth_callback(stream: &mut TcpStream) -> Result<(), TracebackError> {
    // Respond with a success code.
    let message = "Go back to your terminal :)";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
    );
    // Write the response string to the stream.
    match stream.write_all(response.as_bytes()).await {
        Ok(_) => Ok(()),
        Err(e) => Err(traceback!("Failed to write to stream")
            .with_extra_data(json!({"error": e.to_string()}))),
    }
}

#[traceback_derive::traceback]
fn handle_github_token(token_res: TokenResult) -> Result<String, TracebackError> {
    match token_res {
        Ok(token) => {
            // NB: Github returns a single comma-separated "scope" parameter instead of multiple
            // space-separated scopes. Github-specific clients can parse this scope into
            // multiple scopes by splitting at the commas. Note that it's not safe for the
            // library to do this by default because RFC 6749 allows scopes to contain commas.
            let scopes = if let Some(scopes_vec) = token.scopes() {
                scopes_vec
                    .iter()
                    .map(|comma_separated| comma_separated.split(','))
                    .flatten()
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            println!("Github returned the following scopes:\n{:?}\n", scopes);
            Ok(token.access_token().secret().to_string())
        }
        Err(e) => Err(traceback!(err e)),
    }
}

#[traceback_derive::traceback]
fn handle_standard_token(token_res: TokenResult) -> Result<String, TracebackError> {
    match token_res {
        Ok(token) => Ok(token.access_token().secret().to_string()),
        Err(e) => Err(traceback!(err e)),
    }
}

#[traceback_derive::traceback]
async fn get_discord_info(token: &str) -> Result<UserType, TracebackError> {
    Ok(get_info::<DiscordIdentity>(token, "https://discord.com/api/v10/users/@me").await?)
}

#[traceback_derive::traceback]
async fn get_github_info(token: &str) -> Result<UserType, TracebackError> {
    Ok(get_info::<GithubIdentity>(token, "https://api.github.com/user").await?)
}

#[traceback_derive::traceback]
async fn get_info<T>(token: &str, url: &str) -> Result<UserType, TracebackError>
where
    T: Into<UserType> + DeserializeOwned,
{
    let client = reqwest::Client::new();
    let sent = match client
        .get(url)
        .bearer_auth(token)
        .header("User-Agent", "reqwest")
        .send()
        .await
    {
        Ok(sent) => sent,
        Err(e) => return Err(traceback!(err e)),
    };
    let response = sent.text().await?;
    println!("Returned {response}");
    let parsed: GithubIdentity = serde_json::from_str(&response)?;
    Ok(parsed.into())
}

#[tokio::main]
async fn main() {
    let token = oauth(OAuthParams::discord_default()).await.unwrap();
    let user = get_discord_info(&token).await.unwrap();
    let current_time = Utc::now();
    let expiration_time = current_time + Duration::days(30);

    let jwt = JWT {
        user,
        iat: current_time.timestamp(),
        exp: expiration_time.timestamp(),
    };

    let algorithm = Algorithm::RS512;

    let token = encode(
        &Header::new(algorithm),
        &jwt,
        &EncodingKey::from_rsa_pem(PRIVATE_KEY).unwrap(),
    );

    match token {
        Ok(ok_jwt) => {
            println!("JWT: {}", ok_jwt);

            // Define the expected JWT algorithm and validation settings
            let validation = Validation::new(algorithm);

            // Decode and validate the JWT
            match decode::<JWT>(
                &ok_jwt,
                &DecodingKey::from_rsa_pem(PUBLIC_KEY).unwrap(),
                &validation,
            ) {
                Ok(token_data) => {
                    println!("Valid JWT. Claims: {:?}", token_data.claims);
                }
                Err(e) => {
                    eprintln!("Invalid JWT: {}", e);
                }
            }
        }
        Err(e) => eprintln!("Failed to create JWT: {}", e),
    }
}
