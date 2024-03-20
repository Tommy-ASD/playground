use serde::{Deserialize, Serialize};

use crate::payloads::from_client::server::ServerPayload;

mod errors;
mod room;
mod server;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ClientPayload {
    PlainText(String),
    ServerPayload(ServerPayload),
}
