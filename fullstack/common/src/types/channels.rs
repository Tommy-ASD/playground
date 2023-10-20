use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Channel {
    pub id: Uuid,
    pub permitted_users: Vec<Uuid>,
    pub messages: Vec<Uuid>,
}
