use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use crate::payloads::from_server::errors::ErrorPayload;

pub mod errors;
pub mod room;
pub mod server;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ServerPayload {
    PlainText(String),
    BroadcastMessageFromPeer(Uuid, String),
    Error(ErrorPayload),
}
