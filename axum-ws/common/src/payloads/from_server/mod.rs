use serde::{Deserialize, Serialize};

use crate::payloads::from_server::errors::ErrorPayload;

mod errors;
mod room;
mod server;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ServerPayload {
    Error(ErrorPayload),
}
