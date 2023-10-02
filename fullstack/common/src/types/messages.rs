use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::JsValue;
use yew::{html, Component, Context, Html};

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
    pub fn to_html(&self) -> Html {
        html! {
            <p>{ "Messages have no implementation yet" }</p>
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
            sender: "SYSTEM".to_string(),
            content: "Hello, world!".into(),
            sent_at: Utc::now().naive_local(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p>{ "Messages have no implementation yet" }</p>
        }
    }
}

impl From<Message> for JsValue {
    fn from(value: Message) -> Self {
        JsValue::from_serde(&value).unwrap()
    }
}
