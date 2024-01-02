use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Channel {
    id: Uuid,
    permitted_users: Vec<Uuid>,
    messages: Vec<Uuid>,
}
