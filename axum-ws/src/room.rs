use crate::{peer::Peer, state::Payload};
use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct BaseRoom {
    pub id: Uuid,
    pub connected: Vec<Peer>,
    pub payloads: Vec<Payload>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Room {
    Base(BaseRoom),
}
