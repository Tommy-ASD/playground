use crate::{
    peer::Peer,
    state::{AppState, Payload, PayloadDistribution},
};
use std::{collections::HashSet, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use traceback_error::{traceback, TracebackError};
use uuid::Uuid;

mod connected;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum RoomType {
    Base,
}

pub struct Room {
    id: Uuid,
    connected: Mutex<Vec<Uuid>>,
    payloads: Mutex<Vec<Payload>>,
    room_type: RoomType,
    state: Arc<AppState>,
}

impl Room {
    pub async fn new(room_type: RoomType, state: Arc<AppState>) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            connected: Mutex::new(Vec::new()),
            payloads: Mutex::new(Vec::new()),
            room_type,
            state,
        }
    }
    #[traceback_derive::traceback]
    pub async fn send_payload_to_connected(&self, payload: Payload) -> Result<(), TracebackError> {
        let pd = PayloadDistribution {
            receiver_ids: self.get_connected_ids().await,
            payload,
        };
        self.state.send(pd).await?;
        Ok(())
    }
    pub fn get_id(&self) -> Uuid {
        self.id
    }
    pub async fn get_payloads(&self) -> Vec<Payload> {
        self.payloads.lock().await.clone()
    }
    pub fn get_room_type(&self) -> RoomType {
        self.room_type.clone()
    }
}
