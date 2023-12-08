//! Example chat application.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p example-chat
//! ```

use api::make_router;
use axum::{
    extract::{
        ws::{self, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast::{self, Receiver};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use serde_json::{json, Value};

use traceback_error::{traceback, TracebackError};

use common::{Message, Payload, PayloadInner, User};

#[derive(Clone)]
pub struct Sender {
    inner: broadcast::Sender<Payload>,
}

impl Sender {
    #[traceback_derive::traceback]
    fn send(&self, payload: Payload) -> Result<(), TracebackError> {
        self.inner.send(payload)?;

        Ok(())
    }
    pub fn subscribe(&self) -> Receiver<Payload> {
        self.inner.subscribe()
    }
}

// Our shared state
pub struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    user_set: Mutex<HashSet<User>>,
    // Channel used to send payloads to all connected clients.
    tx: Sender,
    // store payloads
    payloads: Mutex<Vec<Payload>>,
}

impl AppState {
    pub fn username_is_unique(&self, name: &str) -> bool {
        return self
            .user_set
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.username == name)
            .is_none();
    }
    /// Returns none if username is already taken
    pub fn add_user(&self, name: &str) -> Option<User> {
        if self.username_is_unique(name) {
            let user = User {
                username: name.to_string(),
            };
            self.user_set.lock().unwrap().insert(user.clone());
            return Some(user);
        }
        None
    }
    #[traceback_derive::traceback]
    pub fn send(&self, payload: Payload) -> Result<(), TracebackError> {
        tracing::debug!("Sending {payload:?}");
        self.tx.send(payload.clone())?;
        let mut payloads = match self.payloads.lock() {
            Ok(payloads) => payloads,
            Err(e) => {
                return Err(
                    TracebackError::new(String::from(""), file!().to_string(), line!())
                        .with_extra_data(json!({
                            "error": e.to_string()
                        })),
                );
            }
        };
        payloads.push(payload.clone());

        Ok(())
    }
    #[traceback_derive::traceback]
    pub fn get_payload_list(&self) -> Result<Vec<Payload>, TracebackError> {
        match self.payloads.lock() {
            Ok(pl) => Ok(pl.iter().map(|pl| pl.clone()).collect::<Vec<Payload>>()),
            Err(e) => Err(traceback!().with_extra_data(json!({"error": e.to_string()}))),
        }
    }
}

pub mod api;

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

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    let pls = Payload::new(PayloadInner::PayloadList(state.get_payload_list().unwrap()));

    sender
        .send(ws::Message::Text(serde_json::to_string(&pls).unwrap()))
        .await
        .unwrap();

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender
                .send(ws::Message::Text(serde_json::to_string(&msg).unwrap()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    let state_clone = state.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(ws::Message::Text(text))) = receiver.next().await {
            tracing::debug!("Recieved message {text}");
            let parsed: Payload = match serde_json::from_str(&text) {
                Ok(p) => p,
                Err(e) => {
                    println!("Failed to parse payload: {e}");
                    continue;
                }
            };
            tracing::debug!("Parsed message {parsed:?}");
            // Add username before message.
            let _ = state_clone.send(parsed);
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../chat.html"))
}
