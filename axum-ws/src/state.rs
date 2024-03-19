use serde::{Deserialize, Serialize};
//allows to split the websocket stream into separate TX and RX branches
use tokio::sync::broadcast::{self};

use traceback_error::{traceback, TracebackError};

use crate::room::Room;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Payload {
    PlainText(String),
}

#[derive(Clone)]
pub struct Sender {
    pub inner: broadcast::Sender<Payload>,
}

impl Sender {
    #[traceback_derive::traceback]
    fn send(&self, payload: Payload) -> Result<(), TracebackError> {
        self.inner.send(payload)?;

        Ok(())
    }
    pub fn subscribe(&self) -> broadcast::Receiver<Payload> {
        self.inner.subscribe()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub tx: Sender,
    pub rooms: Vec<Room>,
}

impl AppState {
    /// Sends a payload to state.tx, which is then received and handled by each peer's respective ws_sender task
    #[traceback_derive::traceback]
    pub async fn send(&self, payload: Payload) -> Result<(), TracebackError> {
        tracing::debug!("Sending {payload:?}");
        println!("Sending and adding {payload:?}");
        self.tx.send(payload.clone())?;

        Ok(())
    }
}
