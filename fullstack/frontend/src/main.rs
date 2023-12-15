use wasm_bindgen::JsValue;
use ws_client::{get_ws_client, setup_client};
use yew::{
    function_component,
    prelude::{html, Component, Context, Html, Properties},
};

use common::{Payload, PayloadInner};

use crate::{state::State, ws_client::callbacks};

mod state;
mod utilities;
mod ws;
mod ws_client;

static BACKEND_URL: &str = "localhost:8081";

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
        let link = ctx.link();
        setup_client(link);

        Self { payloads: vec![] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        gloo::console::log!("Recieved message: ", msg.clone());
        match msg {
            PayloadHandler::AddPayload(item) => {
                match item.inner {
                    PayloadInner::PayloadList(pls) => self.payloads.extend(pls),
                    _ => self.payloads.push(item),
                };
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

        let send = callbacks::send(&link);
        let join = callbacks::join(&link);

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

// Then supply the prop
#[function_component(App)]
fn app() -> Html {
    html! { <PayloadList /> }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
