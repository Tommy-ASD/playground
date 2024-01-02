//! Example chat application.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p example-chat
//! ```

use api::make_router;
use axum::{response::Html, routing::get, Router};
use state::{AppState, Sender};
use std::{collections::HashSet, net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast, Mutex};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use ws::websocket_handler;

mod api;
mod state;
mod ws;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Set up application state for use with with_state().
    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);
    let payloads = Mutex::new(Vec::new());

    let app_state = Arc::new(AppState {
        user_set,
        tx: Sender { inner: tx },
        payloads,
    });

    let api = make_router();

    let app = Router::new()
        .route("/", get(index))
        .route("/websocket", get(websocket_handler))
        .route("/ws", get(websocket_handler))
        .nest("/api", api)
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);
    tokio::spawn(
        hyper::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8081))).serve(app.into_make_service()),
    );
    loop {}
}

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../chat.html"))
}
