use std::{net::SocketAddr, sync::Arc};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::{AppState, Payload};

use traceback_error::{traceback, TracebackError};

/// Represents a connected peer
/// A reference to this struct is given to both ws_sender and ws_recver tasks for each respective peer
#[derive(Clone, Serialize, Deserialize)]
pub struct Peer {
    pub addr: SocketAddr,
    pub id: Uuid,
    #[serde(skip)]
    pub state: Arc<AppState>,
}

impl Peer {
    pub fn get_id(&self) -> Uuid {
        self.id
    }
    #[traceback_derive::traceback]
    pub async fn send(&self, payload: Payload) -> Result<(), TracebackError> {
        self.state.send_payload_to_peer(payload, self.id).await?;
        Ok(())
    }
}
