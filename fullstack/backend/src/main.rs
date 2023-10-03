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
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use serde_json::{json, Value};

use traceback_error::{traceback, TracebackError};

use common::{Message, User};

#[derive(Clone)]
pub struct Sender {
    inner: broadcast::Sender<Message>,
}

impl Sender {
    #[traceback_derive::traceback]
    fn send(&self, message: Message) -> Result<(), TracebackError> {
        self.inner.send(message)?;

        Ok(())
    }
    pub fn subscribe(&self) -> Receiver<Message> {
        self.inner.subscribe()
    }
}

// Our shared state
pub struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    user_set: Mutex<HashSet<User>>,
    // Channel used to send messages to all connected clients.
    tx: Sender,
    // store messages
    messages: Mutex<Vec<Message>>,
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
    pub fn send(&self, message: Message) -> Result<(), TracebackError> {
        self.tx.send(message.clone())?;
        let mut messages = match self.messages.lock() {
            Ok(messages) => messages,
            Err(e) => {
                return Err(
                    TracebackError::new(String::from(""), file!().to_string(), line!())
                        .with_extra_data(json!({
                            "error": e.to_string()
                        })),
                );
            }
        };
        messages.push(message.clone());

        Ok(())
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
    let messages = Mutex::new(Vec::new());

    let app_state = Arc::new(AppState {
        user_set,
        tx: Sender { inner: tx },
        messages,
    });

    let api = make_router();

    let app = Router::new()
        .route("/", get(index))
        .route("/websocket", get(websocket_handler))
        .nest("/api", api)
        .with_state(app_state);
    hyper::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8081)))
        .serve(app.into_make_service())
        .await
        .unwrap();
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

    // Username gets set in the receive loop, if it's valid.
    let mut new_user = None;
    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let ws::Message::Text(name) = message {
            // If username that is sent by client is not taken, fill username string.
            new_user = state.add_user(&name);

            // If not empty we want to quit the loop else we want to quit function.
            if new_user.is_some() {
                break;
            } else {
                // Only send our client that username is taken.
                let _ = sender
                    .send(ws::Message::Text(String::from("Username already taken.")))
                    .await;

                return;
            }
        } else {
            println!("Recieved message: {message:?}");
        }
    }

    let username = new_user.unwrap().username;

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = Message::new(Value::String(format!("{username} joined.")), "SYSTEM");
    tracing::debug!("{msg:?}");
    let _ = state.send(msg);

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

    // Clone things we want to pass (move) to the receiving task.
    let name = username.clone();
    let state_clone = state.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(ws::Message::Text(text))) = receiver.next().await {
            let message = Message::new(Value::String(text), &name);
            // Add username before message.
            let _ = state_clone.send(message);
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Send "user left" message (similar to "joined" above).
    let msg = Message::new(Value::String(format!("{username} left.")), "SYSTEM");
    tracing::debug!("{msg:?}");
    let _ = state.send(msg);

    let mut user_set = state.user_set.lock().unwrap();

    // Remove username from map so new clients can take it again.
    if let Some(user) = user_set.iter().find(|user| user.username == username) {
        let user = user.clone();
        user_set.remove(&user);
    }
}

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../chat.html"))
}
