//! A simple example of hooking up stdin/stdout to a WebSocket stream.
//!
//! This example will connect to a server specified in the argument list and
//! then forward all data read on stdin to the server, printing out all data
//! received on stdout.
//!
//! Note that this is not currently optimized for performance, especially around
//! buffer management. Rather it's intended to show an example of working with a
//! client.
//!
//! You can use this example together with the `server` example.

use std::env;

use common::payloads::{ClientPayload, ServerPayload};
use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

mod input;
pub use input::input_inner;

#[tokio::main]
async fn main() {
    let connect_addr = String::from("ws://localhost:3000/ws");

    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            if let Ok(text) = String::from_utf8(data.clone()) {
                let parsed: ServerPayload = match serde_json::from_str(&text) {
                    Ok(p) => p,
                    Err(e) => todo!(),
                };
                handle_payload(parsed);
            } else {
                todo!()
            }
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    loop {
        let user_in = input!();
        let payload = ClientPayload::PlainText(user_in);
        let text = serde_json::to_string(&payload).unwrap();
        tx.unbounded_send(Message::text(text)).unwrap();
    }
}

fn handle_payload(payload: ServerPayload) {
    match payload {
        ServerPayload::PlainText(txt) => {
            println!("Received plain text from server; {txt}");
        }
        ServerPayload::BroadcastMessageFromPeer(id, txt) => {
            println!(">>> {id}: {txt}");
        }
        ServerPayload::Error(e) => {
            handle_error_payload(e);
        }
    }
}

fn handle_error_payload(payload: common::payloads::from_server::ErrorPayload) {
    todo!();
}
