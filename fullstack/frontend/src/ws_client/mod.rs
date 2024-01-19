use ws::EventClient;
use yew::prelude::html;

use common::{Payload, PayloadInner};

use crate::{
    payload::PayloadHandler,
    payload::PayloadList,
    state::{set_session_id, WS_CLIENT},
    ws, BACKEND_URL,
};

pub mod callbacks;

pub fn setup_client(link: &html::Scope<PayloadList>) {
    let mut client = get_ws_client();

    let on_ws_msg = link.callback(|msg: ws::Message| {
        match msg {
            ws::Message::Text(txtmsg) => {
                gloo::console::log!("Recieved text message from WS: ", &txtmsg);
                let parsed: Payload = serde_json::from_str(&txtmsg).unwrap();

                match parsed.inner.clone() {
                    PayloadInner::Joined(id) => {
                        set_session_id(id.0);
                    }
                    _ => {}
                }

                return PayloadHandler::AddPayload(parsed);
            }
            _ => {
                gloo::console::error!("Got unexpected message format")
            }
        };
        PayloadHandler::None
    });

    client.set_on_message({
        let on_ws_msg = on_ws_msg.clone();
        Some(Box::new(
            move |_client: &ws::EventClient, message: ws::Message| {
                on_ws_msg.emit(message);
            },
        ))
    });
}

pub fn create_client() -> ws::EventClient {
    let mut optional_ws = ws::EventClient::new(&format!("ws://{BACKEND_URL}/ws"));
    while let Err(err) = optional_ws {
        gloo::console::error!("Failed to connect to ws: ", format!("{}", err));
        optional_ws = ws::EventClient::new(&format!("ws://{BACKEND_URL}/ws"));
    }
    gloo::console::log!("Created client");
    optional_ws.unwrap()
}

pub fn get_ws_client() -> EventClient {
    WS_CLIENT.with(|inner| inner.clone())
}
