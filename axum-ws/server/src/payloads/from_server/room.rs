//! Payloads which will be sent to and handled by rooms
//! This would be stuff like sending messages to the room you're currently in

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct ListRooms(Vec<Uuid>);

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum RoomPayload {
    ListRooms(ListRooms),
}
