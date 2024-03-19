use crate::{
    peer::Peer,
    state::{AppState, Payload, PayloadDistribution},
};
use std::{collections::HashSet, sync::Arc};

use serde::{Deserialize, Serialize};
use traceback_error::{traceback, TracebackError};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum RoomType {
    Base,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Room {
    id: Uuid,
    connected: Vec<Peer>,
    payloads: Vec<Payload>,
    room_type: RoomType,
    #[serde(skip)]
    state: Arc<AppState>,
}

impl Room {
    async fn new(room_type: RoomType, state: Arc<AppState>) {}
    #[traceback_derive::traceback]
    async fn send_payload_to_connected(&self, payload: Payload) -> Result<(), TracebackError> {
        let pd = PayloadDistribution {
            receiver_ids: self.get_connected_ids(),
            payload,
        };
        self.state.send(pd).await?;
        Ok(())
    }
    fn get_connected_ids(&self) -> HashSet<Uuid> {
        self.connected.iter().map(|p| p.id).collect()
    }
}
