use axum::{
    response::Html, routing::get, Router, 
};
use std::net::SocketAddr;
use tokio::fs::read_to_string;

async fn serve_index() -> Html<String> {
    // Read the content of the index.html file
    let content = read_to_string("static/index.html").await.expect("index.html file not found");
    Html(content)
}

async fn serve_login() -> Html<String> {
    // Read the content of the index.html file
    let content = read_to_string("static/login.html").await.expect("login.html file not found");
    Html(content)
}

pub async fn serve_axum() {
    // Build our application with a single route
    let app = Router::new().route("/", get(serve_index)).route("/login", get(serve_login));
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:3030"))
    .await
    .unwrap();

    // Run the server
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
