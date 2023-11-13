use std::net::SocketAddr;

use axum::Router;
use tower_http::services::ServeDir;

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
    Router::new().route(path, method_router)
}
