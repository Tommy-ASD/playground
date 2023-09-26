use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    reqwest::{async_http_client, Error as OAuthError},
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, RedirectUrl, RequestTokenError, Scope, StandardErrorResponse,
    StandardTokenResponse, TokenResponse, TokenUrl,
};

use reqwest::Error as ReqwestError;
use url::Url;

use crate::upload::primary;

use traceback_error::{traceback, TracebackError};

mod upload;

type Token = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
type FailedToken =
    RequestTokenError<OAuthError<ReqwestError>, StandardErrorResponse<BasicErrorResponseType>>;
type TokenResult = Result<Token, FailedToken>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().unwrap();

    let token = get_token().await?;

    primary(&token).await.unwrap();

    Ok(())
}

pub fn input_inner() -> String {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => {}
        Err(e) => {
            println!("Error reading line: {}", e);
            println!("Please try again");
            return input_inner();
        }
    }
    line.trim().to_string()
}

#[macro_export]
macro_rules! input {
    ($($arg:expr),*) => {{
        $(print!("{} ", $arg);)* // Print each argument followed by a space
        println!(); // Print a newline at the end

        $crate::input_inner()
    }};
}

#[traceback_derive::traceback]
async fn get_token() -> Result<String, TracebackError> {
    let id = std::env::var("GOOGLE_CLIENT_ID").unwrap();
    let secret = std::env::var("GOOGLE_CLIENT_SECRET").unwrap();
    let client = BasicClient::new(
        ClientId::new(id),
        Some(ClientSecret::new(secret)),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?,
        Some(TokenUrl::new(
            "https://oauth2.googleapis.com/token".to_string(),
        )?),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new("http://localhost:13425".to_string())?);

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/youtube.upload".to_string(),
        ))
        .add_scope(Scope::new("openid".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    let mut token_response: Option<TokenResult> = None;

    println!("Browse to: {}", auth_url);
    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:13425").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            println!("Google returned the following code:\n{}\n", code.secret());
            println!(
                "Google returned the following state:\n{} (expected `{}`)\n",
                state.secret(),
                csrf_token.secret()
            );

            // Exchange the code with a token.
            token_response = Some(
                client
                    .exchange_code(code)
                    .set_pkce_verifier(pkce_verifier)
                    .request_async(async_http_client)
                    .await,
            );

            println!(
                "Google returned the following token:\n{:?}\n",
                token_response
            );

            // The server will terminate itself after revoking the token.
            break;
        }
    }

    let token = token_response
        .unwrap()
        .unwrap()
        .access_token()
        .secret()
        .to_string();
    Ok(token)
}
