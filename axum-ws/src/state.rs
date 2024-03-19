use std::{collections::HashSet, sync::Arc};

use serde::{Deserialize, Serialize};
//allows to split the websocket stream into separate TX and RX branches
use tokio::sync::{broadcast, Mutex};

use traceback_error::{traceback, TracebackError};
use uuid::Uuid;

use crate::{peer::Peer, room::Room};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Payload {
    PlainText(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PayloadDistribution {
    pub receiver_ids: HashSet<Uuid>, // IDs of peers who should receive this payload
    pub payload: Payload,            // The payload to be distributed
}

#[derive(Clone)]
pub struct Sender {
    pub inner: broadcast::Sender<PayloadDistribution>,
}

impl Default for Sender {
    fn default() -> Self {
        Self {
            inner: broadcast::channel(100).0,
        }
    }
}

impl Sender {
    #[traceback_derive::traceback]
    fn send(&self, payload: PayloadDistribution) -> Result<(), TracebackError> {
        self.inner.send(payload)?;

        Ok(())
    }
    pub fn subscribe(&self) -> broadcast::Receiver<PayloadDistribution> {
        self.inner.subscribe()
    }
}

#[derive(Default)]
pub struct AppState {
    pub tx: Sender,
    pub rooms: Mutex<Vec<Arc<Room>>>,
    pub peers: Mutex<Vec<Arc<Peer>>>,
}

impl AppState {
    /// Sends a payload to state.tx, which is then received and handled by each peer's respective ws_sender task
    #[traceback_derive::traceback]
    pub async fn send(&self, payload: PayloadDistribution) -> Result<(), TracebackError> {
        tracing::debug!("Sending {payload:?}");
        println!("Sending and adding {payload:?}");
        self.tx.send(payload.clone())?;

        Ok(())
    }
    pub async fn add_peer_to_room(&self, peer_id: Uuid, room_id: Uuid) {
        if let Some(room) = self.get_room_by_id(room_id).await {
            room.add_user(peer_id).await;
        }
    }
    pub async fn get_room_by_id(&self, id: Uuid) -> Option<Arc<Room>> {
        match self
            .rooms
            .lock()
            .await
            .iter()
            .find(|room| room.get_id() == id)
        {
            Some(room) => Some(Arc::clone(room)),
            None => None,
        }
    }
    pub async fn get_peer_by_id(&self, id: Uuid) -> Option<Arc<Peer>> {
        match self
            .peers
            .lock()
            .await
            .iter()
            .find(|peer| peer.get_id() == id)
        {
            Some(peer) => Some(Arc::clone(peer)),
            None => None,
        }
    }
}
