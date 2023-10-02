use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Message {
    pub content: Value,
    pub sender: String, // refers to username
    pub sent_at: chrono::NaiveDateTime,
}

impl Message {
    pub fn new(content: Value, sender: &str) -> Self {
        Message {
            content,
            sender: sender.to_string(),
            sent_at: Utc::now().naive_local(),
        }
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.sent_at.cmp(&other.sent_at))
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sent_at.cmp(&other.sent_at)
    }
}
