use macros::generate_state;
use wasm_bindgen::JsValue;
use web_sys::HtmlTextAreaElement;
use ws::EventClient;
use yew::{
    function_component,
    prelude::{html, Callback, Component, Context, Html, MouseEvent, NodeRef, Properties},
};

use serde_json::Value;

mod utilities;
mod ws;

use common::{Message, Payload, UserInfo};
use std::{
    ops::DerefMut,
    sync::{Mutex, MutexGuard},
};

static BACKEND_URL: &str = "localhost:8081";

generate_state! {
    message_container_ref,
    username_ref,
    joinbtn_ref,
    textarea_ref,
    input_ref,
    sendbtn_ref,
}

thread_local! {
    pub static WS_CLIENT: EventClient = create_client();
    pub static USERNAME: Mutex<Option<String>> = Mutex::new(None);
}

fn get_username() -> Option<String> {
    USERNAME.with(|inner| {
        inner
            .lock()
            .ok()
            .and_then(|mut opt| Some(opt.deref_mut().clone()))
            .flatten()
    })
}

fn set_username(name: String) {
    USERNAME.with(|inner| {
        inner.lock().ok().map(|mut mutguard_opt| {
            let opt = mutguard_opt.deref_mut();
            *opt = Some(name);
        });
    });
}

fn get_ws_client() -> EventClient {
    WS_CLIENT.with(|inner| inner.clone())
}

#[derive(Debug, Clone)]
enum PayloadHandler {
    AddPayload(Payload),
    None,
}

impl From<PayloadHandler> for JsValue {
    fn from(value: PayloadHandler) -> Self {
        JsValue::from_str(&format!("{:?}", value))
    }
}

#[derive(Properties, PartialEq, Default)]
struct PayloadList {
    payloads: Vec<Payload>,
}

impl Component for PayloadList {
    type Message = PayloadHandler;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut client = get_ws_client();
        let link = ctx.link();
        let on_ws_msg = link.callback(|msg: ws::Message| {
            match msg {
                ws::Message::Text(txtmsg) => {
                    gloo::console::log!("Recieved text message from WS: ", &txtmsg);
                    let parsed: Payload = serde_json::from_str(&txtmsg).unwrap();

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

        Self { payloads: vec![] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        gloo::console::log!("Recieved message: ", msg.clone());
        match msg {
            PayloadHandler::AddPayload(item) => {
                self.payloads.push(item);
                gloo::console::log!("Messages: ");
                self.payloads
                    .iter()
                    .for_each(|message| gloo::console::log!(message.clone()));
                true
            }
            PayloadHandler::None => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        utilities::set_cookie("test", "value");
        let link = ctx.link();

        let State {
            message_container_ref: _,
            username_ref,
            joinbtn_ref,
            textarea_ref,
            input_ref,
            sendbtn_ref,
        } = State::get();

        let send = create_send_callback(&link);
        let join = create_join_callback(&link);

        html! {
            <>
                <input ref={username_ref} id={"username"} style={"display:block; width:100px; box-sizing: border-box"} type={"text"} placeholder={"username"} />
                <button ref={joinbtn_ref} onclick={join} id={"join-chat"} type={"button"}>{ "Join Chat" }</button>
                <table ref={textarea_ref} id={"chat"} style={"display:block; width:600px; height:400px; box-sizing: border-box"} cols={"30"} rows={"10"}>
                {
                    self
                        .payloads
                        .iter()
                        .map(|payload| payload.to_html())
                        .collect::<Vec<Html>>()
                }
                </table>
                <input ref={input_ref} id={"input"} style={"display:block; width:600px; box-sizing: border-box"} type={"text"} placeholder={"chat"} />
                <button ref={sendbtn_ref} id={"send-message"} type={"button"} onclick={send}>{ "Send Message" }</button>
            </>
        }
    }
}

fn create_join_callback(link: &html::Scope<PayloadList>) -> Callback<MouseEvent> {
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

        let _ = get_ws_client().send_string(&value);
        set_username(value);
        return PayloadHandler::None;
    })
}

fn create_send_callback(link: &html::Scope<PayloadList>) -> Callback<MouseEvent> {
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
        PayloadHandler::AddPayload(Payload::new_message(&name, Value::String(value)))
    })
}

// Then supply the prop
#[function_component(App)]
fn app() -> Html {
    html! { <PayloadList /> }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn create_client() -> ws::EventClient {
    let mut optional_ws = ws::EventClient::new(&format!("ws://{BACKEND_URL}/ws"));
    while let Err(err) = optional_ws {
        gloo::console::error!("Failed to connect to ws: ", format!("{}", err));
        optional_ws = ws::EventClient::new(&format!("ws://{BACKEND_URL}/ws"));
    }
    optional_ws.unwrap()
}
