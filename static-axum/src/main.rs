use std::net::SocketAddr;

use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // you can also have two `ServeDir`s nested at different paths
    let serve_dir_from_dist = ServeDir::new("./static/dist");
    let serve_dir_from_router = ServeDir::new("./static/router");

    let router: Router = Router::new()
        .nest_service("/dist", serve_dir_from_dist)
        .nest_service("/router", serve_dir_from_router);

    hyper::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 1049)))
        .serve(router.into_make_service())
        .await
        .unwrap();
}
