use std::{collections::HashSet, sync::Arc};

use common::payloads::from_server::ServerPayload;
use serde::{Deserialize, Serialize};
//allows to split the websocket stream into separate TX and RX branches
use tokio::sync::{broadcast, Mutex};

use traceback_error::{traceback, TracebackError};
use uuid::Uuid;

use crate::{
    peer::Peer,
    room::{Room, RoomType},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PayloadDistribution {
    pub receiver_ids: HashSet<Uuid>, // IDs of peers who should receive this payload
    pub payload: ServerPayload,      // The payload to be distributed
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
    /// Sends a payload to state.tx, which is then received and handled by each peer's respective ws_sender task
    #[traceback_derive::traceback]
    pub async fn add_peer_to_room(
        &self,
        peer_id: Uuid,
        room_id: Uuid,
    ) -> Result<(), TracebackError> {
        if let Some(room) = self.get_room_by_id(room_id).await {
            room.add_user(peer_id).await?;
        }
        Ok(())
    }
    /// Sends a payload to state.tx, which is then received and handled by each peer's respective ws_sender task
    #[traceback_derive::traceback]
    pub async fn remove_peer_from_room(
        &self,
        peer_id: Uuid,
        room_id: Uuid,
    ) -> Result<(), TracebackError> {
        if let Some(room) = self.get_room_by_id(room_id).await {
            room.remove_user(peer_id).await;
        }
        Ok(())
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
    #[traceback_derive::traceback]
    pub async fn broadcast_payload(&self, payload: ServerPayload) -> Result<(), TracebackError> {
        let peer_ids: HashSet<Uuid> = self
            .peers
            .lock()
            .await
            .iter()
            .map(|peer| peer.get_id())
            .collect();
        dbg!();
        println!("Broadcasting {payload:?}");
        println!("to {peer_ids:?}");
        let pd = PayloadDistribution {
            payload,
            receiver_ids: peer_ids,
        };
        self.tx.send(pd)?;
        Ok(())
    }
    #[traceback_derive::traceback]
    pub async fn send_payload_to_peer(
        &self,
        payload: ServerPayload,
        peer_id: Uuid,
    ) -> Result<(), TracebackError> {
        let peer_ids = [peer_id].into_iter().collect();
        let pd = PayloadDistribution {
            payload,
            receiver_ids: peer_ids,
        };
        self.tx.send(pd)?;
        Ok(())
    }
    pub async fn add_room(self: Arc<Self>, room_type: RoomType) -> Arc<Room> {
        let room = Room::new(room_type, Arc::clone(&self)).await;
        let arc = Arc::new(room);
        self.rooms.lock().await.push(Arc::clone(&arc));
        arc
    }
}
