use yew::prelude::{html, Callback, MouseEvent};

use serde_json::Value;

use common::Payload;

use crate::{
    get_ws_client, state::get_username, state::set_username, PayloadHandler, PayloadList, State,
};

pub fn join(link: &html::Scope<PayloadList>) -> Callback<MouseEvent> {
    let username_ref = State::get_username_ref();
    link.callback(move |_event: MouseEvent| {
        gloo::console::log!("Button pressed");
        let value = match username_ref.cast::<web_sys::HtmlInputElement>() {
            Some(element) => element.value(),
            None => {
                gloo::console::log!("No input was provided");
                return PayloadHandler::None;
            }
        };

        let pl = Payload::new_joined(&value);

        let _ = get_ws_client().send_payload(&pl);
        set_username(value);
        return PayloadHandler::None;
    })
}

pub fn send(link: &html::Scope<PayloadList>) -> Callback<MouseEvent> {
    let input_ref = State::get_input_ref();
    link.callback(move |_event: MouseEvent| {
        let name = get_username();
        if name.is_none() {
            return PayloadHandler::None;
        }
        let name = name.unwrap();
        gloo::console::log!("Button pressed");
        let value = match input_ref.cast::<web_sys::HtmlInputElement>() {
            Some(element) => element.value(),
            None => {
                gloo::console::log!("No input was provided");
                return PayloadHandler::None;
            }
        };
        gloo::console::log!("Got message ", &value);
        let pl = Payload::new_message(&name, Value::String(value));
        let _ = get_ws_client().send_payload(&pl);
        return PayloadHandler::None;
    })
}
