use crate::AppState;
use axum::{
    extract::{
        ws::{self, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures::{
    sink::SinkExt,
    stream::{SplitStream, StreamExt},
};
use std::sync::Arc;

use common::{reexports::uuid::Uuid, JoinAck, Payload, PayloadInner};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    println!("Websocket handler called");
    ws.on_upgrade(|socket| websocket(socket, state))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    // "sender" sends messages to the user
    // "reciever" recieves messages from the user
    let (mut sender, receiver) = stream.split();

    let session_id = Uuid::new_v4();

    sender
        .send(ws::Message::Text(
            serde_json::to_string(&Payload::new_joined(session_id)).unwrap(),
        ))
        .await
        .unwrap();

    // get all existing payloads
    let plst = Payload::new(PayloadInner::PayloadList(state.get_payload_list().await));

    println!("Got payload: {plst:?}");

    // send payloads to user
    sender
        .send(ws::Message::Text(serde_json::to_string(&plst).unwrap()))
        .await
        .unwrap();

    let state_clone = state.clone();

    // Manager which sends all messages received on state.tx to user
    let mut send_task = tokio::spawn(ws_send_task(sender, state_clone));

    let state_clone = state.clone();

    // Manager which takes all payloads sent from user and sends them to all others using state.send()
    let mut recv_task = tokio::spawn(ws_recv_task(receiver, state_clone));

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

async fn ws_recv_task(mut receiver: SplitStream<WebSocket>, state: Arc<AppState>) {
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
        match state.send(parsed).await {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}

async fn ws_send_task(
    mut sender: futures::prelude::stream::SplitSink<WebSocket, ws::Message>,
    state: Arc<AppState>,
) {
    let mut rx: tokio::sync::broadcast::Receiver<Payload> = state.tx.subscribe();

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
}
