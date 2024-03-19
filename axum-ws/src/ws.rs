use crate::{
    peer::Peer,
    state::{AppState, Payload},
};
use axum::{
    extract::{
        connect_info::ConnectInfo,
        ws::{self, Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use axum_extra::{headers, TypedHeader};
use traceback_error::{traceback, TracebackError};
use uuid::Uuid;

use std::{net::SocketAddr, sync::Arc};

//allows to split the websocket stream into separate TX and RX branches
use futures::{
    sink::SinkExt,
    stream::{SplitStream, StreamExt},
};

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(socket: WebSocket, who: SocketAddr, state: Arc<AppState>) {
    let peer = Peer {
        addr: who,
        id: Uuid::new_v4(),
    };
    let peer = Arc::new(peer);
    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (sender, receiver) = socket.split();

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(ws_sender(Arc::clone(&peer), sender, Arc::clone(&state)));

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(ws_recver(Arc::clone(&peer), receiver, Arc::clone(&state)));

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(()) => println!("Connection to {who} terminated"),
                Err(a) => println!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(()) => println!("Shut down recv task"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    println!("Websocket context {who} destroyed");
}

/// Sender task attached to each websocket connection, handles all sent messages
async fn ws_sender(
    peer: Arc<Peer>,
    mut sender: futures::prelude::stream::SplitSink<WebSocket, ws::Message>,
    state: Arc<AppState>,
) {
    let mut rx: tokio::sync::broadcast::Receiver<Payload> = state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        // for each message sent in state.tx
        // In any websocket error, break loop.
        if sender
            .send(ws::Message::Text(serde_json::to_string(&msg).unwrap()))
            .await
            .is_err()
        {
            break;
        }
    }
}

/// Receiver task attached to each websocket connection, handles all received messages
async fn ws_recver(peer: Arc<Peer>, mut receiver: SplitStream<WebSocket>, state: Arc<AppState>) {
    while let Some(Ok(msg)) = receiver.next().await {
        // for each received message
        match message_received_from_peer(msg, Arc::clone(&state), Arc::clone(&peer)).await {
            Ok(()) => {}
            Err(e) => {
                traceback!(err e);
            }
        };
    }
}

#[traceback_derive::traceback]
async fn message_received_from_peer(
    msg: Message,
    state: Arc<AppState>,
    peer: Arc<Peer>,
) -> Result<(), TracebackError> {
    match msg {
        Message::Text(txt) => {
            let parsed: Payload = serde_json::from_str(&txt)?;
            state.send(parsed).await?;
        }
        _ => {}
    }
    Ok(())
}
