use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use wasm_bindgen::JsValue;
use yew::{html, virtual_dom::VNode, Html};

use crate::Message;

// Represents username
// Will remain as a struct for later flexibility
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct UserInfo(pub String);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct LeaveInfo(pub String);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MessageInfo {
    content: Value,
    sender: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum PayloadInner {
    Joined(UserInfo),
    Left(LeaveInfo),
    Message(MessageInfo),
    PayloadList(Vec<Payload>),
}

impl PayloadInner {
    pub fn new_joined(username: &str) -> Self {
        Self::Joined(UserInfo(username.to_string()))
    }
    pub fn new_left(username: &str) -> Self {
        Self::Left(LeaveInfo(username.to_string()))
    }
    pub fn new_message(sender: &str, content: Value) -> Self {
        Self::Message(MessageInfo {
            content,
            sender: sender.to_owned(),
        })
    }

    pub fn display(&self) -> String {
        match self {
            PayloadInner::Joined(info) => format!("{name} joined", name = info.0),
            PayloadInner::Left(name) => format!("{name} joined", name = name.0),
            PayloadInner::Message(msg) => {
                format!("{name}: {msg}", name = msg.sender, msg = msg.content)
            }
            PayloadInner::PayloadList(list) => list
                .iter()
                .map(|pl| match pl.to_html() {
                    VNode::VRaw(raw) => raw.html.to_string(),
                    _ => unimplemented!(),
                })
                .collect::<Vec<String>>()
                .join("\n"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PayloadMeta {
    pub id: Uuid,
    pub sent_at: chrono::NaiveDateTime,
}

impl PayloadMeta {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            sent_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Payload {
    pub inner: PayloadInner,
    pub meta: PayloadMeta,
}

impl Payload {
    pub fn new(inner: PayloadInner) -> Self {
        Self {
            inner,
            meta: PayloadMeta::new(),
        }
    }
    pub fn new_joined(username: &str) -> Self {
        Self {
            inner: PayloadInner::new_joined(username),
            meta: PayloadMeta::new(),
        }
    }
    pub fn new_left(username: &str) -> Self {
        Self {
            inner: PayloadInner::new_left(username),
            meta: PayloadMeta::new(),
        }
    }
    pub fn new_message(sender: &str, content: Value) -> Self {
        Self {
            inner: PayloadInner::new_message(sender, content),
            meta: PayloadMeta::new(),
        }
    }
    pub fn to_html(&self) -> Html {
        let PayloadMeta {
            id: _,
            sent_at: timestamp,
        } = self.meta;
        let contents = self.inner.display();
        html! {
            <p>{ format!("({timestamp})\n{contents}") }</p>
        }
    }
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl From<Payload> for JsValue {
    fn from(value: Payload) -> Self {
        JsValue::from_serde(&value).unwrap()
    }
}
