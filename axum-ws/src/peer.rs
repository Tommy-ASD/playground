use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a connected peer
/// A reference to this struct is given to both ws_sender and ws_recver tasks for each respective peer
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Peer {
    pub addr: SocketAddr,
    pub id: Uuid,
}
