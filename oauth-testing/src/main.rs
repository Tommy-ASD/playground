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

use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    reqwest::{async_http_client, Error as OAuthError},
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    RedirectUrl, RequestTokenError, RevocationErrorResponseType, Scope, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse,
    TokenResponse, TokenUrl,
};

use reqwest::Error as ReqwestError;
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

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    discord_oauth(&vec![Scope::new("identify".to_string())])
        .await
        .unwrap();
}

#[derive(Clone, Debug)]
pub struct OAuth2Info {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
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

async fn oauth(params: OAuthParams) -> Result<(), TracebackError> {
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
    tcp_server(client, csrf_state, params).await?;

    Ok(())
}

async fn tcp_server(
    client: ClientWithGenerics,
    csrf_state: CsrfToken,
    params: OAuthParams,
) -> Result<(), TracebackError> {
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
                "{provider_name} returned the following token:\n{:?}\n",
                token_res,
                provider_name = params.provider_name
            );
            match params.oauth_type {
                OAuthType::Standard => handle_standard_token(token_res)?,
                OAuthType::Github => handle_github_token(token_res)?,
            };

            // The server will terminate itself after collecting the first code.
            return Ok(());
        }
    }
}

async fn discord_oauth(scopes: &Vec<Scope>) -> Result<(), TracebackError> {
    let github_client_id = dotenv!("DISCORD_CLIENT_ID");
    let github_client_secret = dotenv!("DISCORD_CLIENT_SECRET");
    let auth_url = "https://discord.com/api/oauth2/authorize".to_string();
    let token_url = "https://discord.com/api/oauth2/token".to_string();

    let info = OAuth2Info {
        client_id: github_client_id.to_string(),
        client_secret: github_client_secret.to_string(),
        auth_url,
        token_url,
        redirect_url: "http://localhost:8080/oauth_callback".to_string(),
    };

    let params = OAuthParams {
        provider_name: "Discord".to_string(),
        general_info: info,
        scopes: scopes.clone(),
        oauth_type: OAuthType::Standard,
    };

    oauth(params).await?;

    Ok(())
}

async fn github_oauth(scopes: &Vec<Scope>) -> Result<(), TracebackError> {
    let github_client_id = dotenv!("GITHUB_CLIENT_ID");
    let github_client_secret = dotenv!("GITHUB_CLIENT_SECRET");
    let auth_url = "https://github.com/login/oauth/authorize".to_string();
    let token_url = "https://github.com/login/oauth/access_token".to_string();

    let info = OAuth2Info {
        client_id: github_client_id.to_string(),
        client_secret: github_client_secret.to_string(),
        auth_url,
        token_url,
        redirect_url: "http://localhost:8080/oauth_callback".to_string(),
    };

    let params = OAuthParams {
        provider_name: "Github".to_string(),
        general_info: info,
        scopes: scopes.clone(),
        oauth_type: OAuthType::Github,
    };

    oauth(params).await?;

    Ok(())
}

async fn parse_oauth_callback_request(
    stream: &mut TcpStream,
) -> Result<(AuthorizationCode, CsrfToken), TracebackError> {
    let mut reader = BufReader::new(stream);

    let mut request_line = String::new();
    // Read the first line of the stream.
    // Reads until a newline is encountered (which is discarded).
    // Result should be "GET /oauth_callback?code={code} HTTP/1.1\r\n".
    reader.read_line(&mut request_line).await.unwrap();

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

fn handle_github_token(token_res: TokenResult) -> Result<(), TracebackError> {
    if let Ok(token) = token_res {
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
        println!("Token: {}", token.access_token().secret())
    } else if let Err(e) = token_res {
        let e = e;
        println!("Token exchange failed: {}", e);
    }
    Ok(())
}

fn handle_standard_token(token_res: TokenResult) -> Result<(), TracebackError> {
    if let Ok(token) = token_res {
        let access_token = token.access_token().secret();
        println!("Token: {}", access_token);
    } else if let Err(e) = token_res {
        let e = e;
        println!("Token exchange failed: {}", e);
    }
    Ok(())
}
