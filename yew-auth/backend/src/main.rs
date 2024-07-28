use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use axum::{extract::Query, response::Redirect, routing::get, Json, Router};
use tower_http::services::ServeDir;

use serde::Deserialize;

use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    reqwest::{async_http_client, Error as OAuthError},
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeVerifier, RedirectUrl, RequestTokenError, RevocationErrorResponseType, Scope,
    StandardErrorResponse, StandardRevocableToken, StandardTokenIntrospectionResponse,
    StandardTokenResponse, TokenResponse, TokenUrl,
};

use reqwest::Error as ReqwestError;

type ClientWithGenerics = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
>;

type Token = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

type TokenResult = Result<
    Token,
    RequestTokenError<OAuthError<ReqwestError>, StandardErrorResponse<BasicErrorResponseType>>,
>;

use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    // you can also have two `ServeDir`s nested at different paths
    let index = ServeDir::new("./dist");
    let router: Router = Router::new()
        .nest("/api", api())
        .nest("/authorized", authorized())
        .nest_service("/", index);
    println!("Localhost: {:?}", Ipv6Addr::LOCALHOST);
    let binding_addresses: Vec<SocketAddr> = vec![
        SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8080).into(),
        SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 8080, 0, 0).into(),
    ];
    binding_addresses.iter().for_each(|address| {
        tokio::task::spawn(hyper::Server::bind(address).serve(router.clone().into_make_service()));
    });
    loop {}
}

fn authorized() -> Router {
    Router::new().route("/github", get(authorized_github))
}

async fn authorized_github() -> Redirect {
    Redirect::to("/")
}

fn api() -> Router {
    Router::new().nest("/oauth_callback", oauth_callbacks())
}

fn oauth_callbacks() -> Router {
    Router::new().route("/github", get(oauth_callback))
}

#[derive(Deserialize)]
struct OAuthQuery {
    code: String,
    state: String,
}

async fn oauth_callback(Query(OAuthQuery { code, state }): Query<OAuthQuery>) -> Json<Token> {
    println!("Got code {code}");
    let client = BasicClient::new(
        ClientId::new(dotenv!("GITHUB_CLIENT_ID").to_string()),
        Some(ClientSecret::new(
            dotenv!("GITHUB_CLIENT_SECRET").to_string(),
        )),
        AuthUrl::new("http://auth".to_string()).expect("This doesn't really matter"),
        Some(
            TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                .expect("This does matter"),
        ),
    )
    .set_redirect_uri(RedirectUrl::from_url(
        url::Url::parse("http://localhost:8080/authorized/github").unwrap(),
    ));
    let pkce_verifier = PkceCodeVerifier::new(state);
    let token_result: TokenResult = client
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;

    match token_result {
        Err(err) => {
            println!("{:?}", err.to_string());
            match err {
                RequestTokenError::Other(o) => println!("Other: {o}"),
                RequestTokenError::Parse(o, a) => {
                    println!("Parse: {o:?}");
                    println!("{}", String::from_utf8_lossy(a.as_slice()))
                }
                RequestTokenError::Request(o) => println!("Request: {o:?}"),
                RequestTokenError::ServerResponse(o) => {
                    println!("ServerResponse: {o:?}");
                    let e = o.error();
                    match e {
                        BasicErrorResponseType::InvalidClient => println!("InvalidClient"),
                        BasicErrorResponseType::InvalidGrant => println!("InvalidGrant"),
                        BasicErrorResponseType::InvalidRequest => println!("InvalidRequest"),
                        BasicErrorResponseType::InvalidScope => println!("InvalidScope"),
                        BasicErrorResponseType::UnauthorizedClient => {
                            println!("UnauthorizedClient")
                        }
                        BasicErrorResponseType::UnsupportedGrantType => {
                            println!("UnsupportedGrantType")
                        }
                        BasicErrorResponseType::Extension(ext) => println!("Extension: {ext}"),
                    };
                    println!("Error desc: {:?}", o.error_description());
                    println!("Error uri: {:?}", o.error_uri());
                }
            }
            panic!("TODO better error handling here");
        }
        Ok(val) => {
            println!("Tokens received from OAuth provider!");
            Json(val)
        }
    }
}
