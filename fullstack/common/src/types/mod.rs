use std::collections::HashMap;

mod messages;
pub use messages::*;
mod channels;
pub use channels::*;
use uuid::Uuid;

pub struct State {
    pub messages: Vec<Message>,
    pub channels: Vec<Channel>,

    pub message_lookup: HashMap<Uuid, Message>,
    pub channel_lookup: HashMap<Uuid, Channel>,
}
