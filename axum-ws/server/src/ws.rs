use crate::{
    peer::Peer,
    state::{AppState, PayloadDistribution},
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
use common::payloads::ClientPayload;
use serde_json::json;
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
        state: Arc::clone(&state),
    };
    dbg!();
    println!("ID set to {id}", id = peer.get_id());
    let peer = Arc::new(peer);
    state.peers.lock().await.push(Arc::clone(&peer));
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

/// Sender task attached to each websocket connection, handles all messages sent to the peer
async fn ws_sender(
    peer: Arc<Peer>,
    mut sender: futures::prelude::stream::SplitSink<WebSocket, ws::Message>,
    state: Arc<AppState>,
) {
    let mut rx: tokio::sync::broadcast::Receiver<PayloadDistribution> = state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        if !msg.receiver_ids.contains(&peer.id) {
            println!(
                "Payload {msg:?} does not include user ID {id}",
                id = peer.id
            );
            continue;
        }
        println!("Sending {msg:?} to {id}", id = peer.id);
        // for each message sent in state.tx
        // In any websocket error, break loop.
        if sender
            .send(ws::Message::Text(
                serde_json::to_string(&msg.payload).unwrap(),
            ))
            .await
            .is_err()
        {
            println!(
                "Failed sending {msg:?} to {id}, closing connection",
                id = peer.id
            );
            break;
        }
    }
}

/// Receiver task attached to each websocket connection, handles all messages sent by the peer
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
            handle_text_message_from_peer(txt, state, peer).await?;
        }
        Message::Binary(bytes) => {
            if let Ok(txt) = String::from_utf8(bytes) {
                handle_text_message_from_peer(txt, state, peer).await?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[traceback_derive::traceback]
async fn handle_text_message_from_peer(
    txt: String,
    state: Arc<AppState>,
    peer: Arc<Peer>,
) -> Result<(), TracebackError> {
    println!("Broadcasting {txt} to all users");
    // let parsed: Payload = match serde_json::from_str(&txt) {
    //     Ok(p) => p,
    //     Err(e) => {
    //         return Err(
    //             traceback!(err e, "Failed to parse JSON").with_extra_data(json!({"json": txt}))
    //         )
    //     }
    // };
    let parsed = ClientPayload::PlainText(txt);
    dbg!();
    println!("Received message. Parsed; {parsed:?}");
    // state.broadcast_payload(parsed).await?;
    Ok(())
}
