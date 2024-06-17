//! Payloads which will be sent to and handled by the main server
//! This would be stuff like joining/leaving rooms

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ServerPayload {
    JoinRoom(Uuid),
    LeaveRoom,
}
