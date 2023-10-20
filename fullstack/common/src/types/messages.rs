use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use wasm_bindgen::JsValue;
use yew::{html, Component, Context, Html};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Sender {
    System,
    User(String), // refers to username
}

impl std::fmt::Display for Sender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Sender::System => write!(f, "System"),
            Sender::User(username) => write!(f, "User({})", username),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: Uuid,
    pub content: Value,
    pub sender: Sender,
    pub sent_at: chrono::NaiveDateTime,
}

impl Message {
    pub fn new(content: Value, sender: &str) -> Self {
        Message {
            id: Uuid::new_v4(),
            content,
            sender: Sender::User(sender.to_string()),
            sent_at: Utc::now().naive_local(),
        }
    }
    pub fn new_system(content: Value) -> Self {
        Message {
            id: Uuid::new_v4(),
            content,
            sender: Sender::System,
            sent_at: Utc::now().naive_local(),
        }
    }
    pub fn to_html(&self) -> Html {
        html! {
            <p>{ format!("({timestamp})\n{sender}: {content}", timestamp = self.sent_at, sender = self.sender, content = self.content) }</p>
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

impl Component for Message {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender: Sender::System,
            content: "Hello, world!".into(),
            sent_at: Utc::now().naive_local(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.to_html()
    }
}

impl From<Message> for JsValue {
    fn from(value: Message) -> Self {
        JsValue::from_serde(&value).unwrap()
    }
}
