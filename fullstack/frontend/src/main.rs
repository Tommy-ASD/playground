use wasm_bindgen::JsValue;
use yew::{
    prelude::{html, Component, Context, Html},
    MouseEvent, NodeRef,
};

use serde_json::Value;

mod utilities;
mod ws;

use common::Message;

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

struct MessageList {
    messages: Vec<Message>,
}

impl Component for MessageList {
    type Message = ChangeTodoList;
    type Properties = ();

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
        // setup_websocket!();
        utilities::set_cookie("test", "value");
        let link = ctx.link();

        let (username_ref, joinbtn_ref, textarea_ref, input_ref, sendbtn_ref) = (
            NodeRef::default(),
            NodeRef::default(),
            NodeRef::default(),
            NodeRef::default(),
            NodeRef::default(),
        );

        let send;
        {
            let input_ref = input_ref.clone();
            send = link.callback(move |_event: MouseEvent| {
                gloo::console::log!("Button pressed");
                let value = match input_ref.cast::<web_sys::HtmlInputElement>() {
                    Some(element) => element.value(),
                    None => {
                        gloo::console::log!("No input was provided");
                        return ChangeTodoList::None;
                    }
                };
                ChangeTodoList::AddMessage(Message::new(Value::String(value), "test"))
            });
        };
        let rendered_textarea = self
            .messages
            .iter()
            .map(|message| {
                gloo::console::log!("Rendering message: ", message.clone());
                message.content.to_string()
            })
            .collect::<String>();

        html! {
            <>
                <input ref={username_ref} id={"username"} style={"display:block; width:100px; box-sizing: border-box"} type={"text"} placeholder={"username"} />
                <button ref={joinbtn_ref} id={"join-chat"} type={"button"}>{ "Join Chat" }</button>
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
            </>
        }
    }
}

fn main() {
    yew::Renderer::<MessageList>::new().render();
}

#[macro_export]
macro_rules! setup_websocket {
    () => {
        let mut client = ws::EventClient::new("ws://localhost:8081/websocket").unwrap();
        client.set_on_error(Some(Box::new(|error| {
            gloo::console::error!(error);
        })));
        client.set_on_connection(Some(Box::new(|client: &ws::EventClient| {
            gloo::console::log!(client.get_status());
            gloo::console::log!("Sending message...");
            client.send_string("Hello, World!").unwrap();
        })));
        client.set_on_close(Some(Box::new(|_evt| {
            gloo::console::log!("Connection closed");
        })));
        client.set_on_message(Some(Box::new(
            |_client: &ws::EventClient, message: ws::Message| {
                gloo::console::log!("New Message: ", message);
            },
        )));
    };
}
