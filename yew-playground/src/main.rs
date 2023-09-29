use chrono::{NaiveDateTime, Utc};
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::{
    function_component,
    prelude::{html, use_node_ref, Component, Context, Html},
    Callback, Hook, InputEvent, MouseEvent, NodeRef, Properties,
};

struct TodoItem {
    added_at: NaiveDateTime,
    text: String,
}

impl TodoItem {
    fn new() -> Self {
        Self {
            added_at: Utc::now().naive_local(),
            text: "No text specified".to_string(),
        }
    }
    fn with_text(text: String) -> Self {
        Self {
            added_at: Utc::now().naive_local(),
            text,
        }
    }
    fn to_html(&self) -> Html {
        html! {
            <li>
                <p>{ &self.added_at }</p>
                <p>{ &self.text }</p>
            </li>
        }
    }
}

impl Component for TodoItem {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            added_at: Utc::now().naive_local(),
            text: "No text specified".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <li>
                <p>{ &self.added_at }</p>
                <p>{ &self.text }</p>
            </li>
        }
        /*
            <input oninput={ctx.link().callback(|item: InputEvent| {
                ChangeTodoItem::ChangeText(item.data().unwrap())
            })}/>
        */
    }
}

enum ChangeTodoList {
    AddItem(TodoItem),
    RemoveItem(usize),
}

struct TodoList {
    items: Vec<TodoItem>,
}

impl Component for TodoList {
    type Message = ChangeTodoList;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { items: vec![] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChangeTodoList::AddItem(item) => {
                self.items.push(item);
                true
            }
            ChangeTodoList::RemoveItem(index) => {
                self.items.remove(index);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
            <ul>
                { self.items.iter().enumerate().map(|(index, item)| {
                    html! {
                        <>
                            {item.to_html()}
                            <button onclick={link.callback(move |_event: MouseEvent| {
                                ChangeTodoList::RemoveItem(index)
                            })}/>
                        </>
                    }
                }).collect::<Vec<Html>>() }
            </ul>
            <input
                id={"TodoListInput"}
                onchange = {
                    link.callback(|event: Event| {
                        let target: web_sys::EventTarget = event.target().unwrap();
                        let input = target.unchecked_into::<HtmlInputElement>();
                        let value = input.value();
                        log!(&value);
                        ChangeTodoList::AddItem(TodoItem::with_text(value))
                    })
                }
            />
            </>
        }
    }
}

fn main() {
    yew::Renderer::<TodoList>::new().render();
}
