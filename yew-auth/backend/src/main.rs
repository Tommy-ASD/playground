use std::net::SocketAddr;

use axum::{extract::Query, routing::get, Router};
use tower_http::services::ServeDir;

use serde::Deserialize;

#[tokio::main]
async fn main() {
    // you can also have two `ServeDir`s nested at different paths
    let index = ServeDir::new("./dist");

    let router: Router = Router::new().nest("/api", api()).nest_service("/", index);

    hyper::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8080)))
        .serve(router.into_make_service())
        .await
        .unwrap();
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

async fn oauth_callback(Query(OAuthQuery { code, .. }): Query<OAuthQuery>) {
    println!("Got code {code}");
    let client = BasicClient::new(
        ClientId::new("".to_string()),
        Some(ClientSecret::new("".to_string())),
        AuthUrl::new("http://auth".to_string()).expect("This doesn't really matter"),
        Some(TokenUrl::new("http://fixthis").expect("This does matter")),
    );
}
