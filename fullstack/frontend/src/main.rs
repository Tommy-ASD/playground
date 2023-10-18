use macros::generate_state;
use wasm_bindgen::JsValue;
use web_sys::HtmlTextAreaElement;
use ws::EventClient;
use yew::{
    function_component,
    prelude::{html, Callback, Component, Context, Html, MouseEvent, NodeRef, Properties},
};

use serde_json::Value;

mod canvas;
mod utilities;
mod ws;

use common::Message;

use crate::canvas::func_plot::draw;

generate_state! {
    message_container_ref,
    username_ref,
    joinbtn_ref,
    textarea_ref,
    input_ref,
    sendbtn_ref,
    canvas_ref,
    renderbtn_ref,
}

thread_local! {
    pub static WS_CLIENT: EventClient = create_client();
}

fn get_ws_client() -> EventClient {
    WS_CLIENT.with(|inner| inner.clone())
}

#[derive(Debug, Clone)]
enum ChangeTodoList {
    AddMessage(Message),
    RemoveItem(usize),
    None,
}

impl From<ChangeTodoList> for JsValue {
    fn from(value: ChangeTodoList) -> Self {
        JsValue::from_str(&format!("{:?}", value))
    }
}

#[derive(Properties, PartialEq)]
struct WsCallbacks {
    pub on_message_recieved: Callback<ws::Message>,
}

#[derive(Properties, PartialEq)]
struct MessageListProps {
    pub ws_callbacks: WsCallbacks,
}

#[derive(Properties, PartialEq, Default)]
struct MessageList {
    messages: Vec<Message>,
}

impl Component for MessageList {
    type Message = ChangeTodoList;
    type Properties = MessageListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { messages: vec![] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        gloo::console::log!("Recieved message: ", msg.clone());
        match msg {
            ChangeTodoList::AddMessage(item) => {
                self.messages.push(item);
                gloo::console::log!("Messages: ");
                self.messages
                    .iter()
                    .for_each(|message| gloo::console::log!(message.clone()));
                true
            }
            ChangeTodoList::RemoveItem(index) => {
                self.messages.remove(index);
                true
            }
            ChangeTodoList::None => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        utilities::set_cookie("test", "value");
        let link = ctx.link();

        let State {
            message_container_ref,
            username_ref,
            joinbtn_ref,
            textarea_ref,
            input_ref,
            sendbtn_ref,
            canvas_ref,
            renderbtn_ref,
        } = State::get();

        let send = create_send_callback(&link, &input_ref);
        let join = create_join_callback(&link, &username_ref, &get_ws_client());
        let render = create_render_callback(&link);

        html! {
            <>
                <input ref={username_ref} id={"username"} style={"display:block; width:100px; box-sizing: border-box"} type={"text"} placeholder={"username"} />
                <button ref={joinbtn_ref} onclick={join} id={"join-chat"} type={"button"}>{ "Join Chat" }</button>
                <table ref={textarea_ref} id={"chat"} style={"display:block; width:600px; height:400px; box-sizing: border-box"} cols={"30"} rows={"10"}>
                {
                    self
                        .messages
                        .iter()
                        .map(|message| message.to_html())
                        .collect::<Vec<Html>>()
                }
                </table>
                <input ref={input_ref} id={"input"} style={"display:block; width:600px; box-sizing: border-box"} type={"text"} placeholder={"chat"} />
                <button ref={sendbtn_ref} id={"send-message"} type={"button"} onclick={send}>{ "Send Message" }</button>
                <canvas ref={canvas_ref} id={"canvas"} />
                <button ref={renderbtn_ref} type={"button"} onclick={render}>{ "Render canvas" }</button>
            </>
        }
    }
}

fn create_join_callback(
    link: &html::Scope<MessageList>,
    username_ref: &NodeRef,
    client: &EventClient,
) -> Callback<MouseEvent> {
    let client: EventClient = client.clone();
    let username_ref = username_ref.clone();
    link.callback(move |_event: MouseEvent| {
        gloo::console::log!("Button pressed");
        let value = match username_ref.cast::<web_sys::HtmlInputElement>() {
            Some(element) => element.value(),
            None => {
                gloo::console::log!("No input was provided");
                return ChangeTodoList::None;
            }
        };
        let _ = client.send_string(&value);
        return ChangeTodoList::None;
    })
}

fn create_send_callback(
    link: &html::Scope<MessageList>,
    input_ref: &NodeRef,
) -> Callback<MouseEvent> {
    let input_ref = input_ref.clone();
    link.callback(move |_event: MouseEvent| {
        gloo::console::log!("Button pressed");
        let value = match input_ref.cast::<web_sys::HtmlInputElement>() {
            Some(element) => element.value(),
            None => {
                gloo::console::log!("No input was provided");
                return ChangeTodoList::None;
            }
        };
        ChangeTodoList::AddMessage(Message::new(Value::String(value), "test"))
    })
}

fn create_render_callback(link: &html::Scope<MessageList>) -> Callback<MouseEvent> {
    link.callback(move |_event: MouseEvent| {
        gloo::console::log!("Button pressed");
        let _ = draw("canvas", 0);
        ChangeTodoList::None
    })
}

// Then supply the prop
#[function_component(App)]
fn app() -> Html {
    let mut client = get_ws_client();
    let on_message_recieved: Callback<ws::Message> = Callback::from(move |name: ws::Message| {
        let greeting = format!("Hey, {name:?}!");
    });
    client.set_on_message({
        let on_message_recieved = on_message_recieved.clone();
        Some(Box::new(
            move |_client: &ws::EventClient, message: ws::Message| {
                on_message_recieved.emit(message);
            },
        ))
    });

    html! { <MessageList ws_callbacks={WsCallbacks {on_message_recieved}}/> }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn create_client() -> ws::EventClient {
    let mut client: ws::EventClient =
        ws::EventClient::new("ws://localhost:8081/websocket").unwrap();
    client
}
