use std::{net::SocketAddr, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use traceback_error::{traceback, TracebackError};
use uuid::Uuid;

use crate::state::{AppState, Payload};

/// Represents a connected peer
/// A reference to this struct is given to both ws_sender and ws_recver tasks for each respective peer
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Peer {
    pub addr: SocketAddr,
    pub id: Uuid,
}

pub struct PeerHandler {
    pub peer: Arc<Peer>,
    pub connected_room_id: Uuid,
    pub state: Arc<AppState>,
}
