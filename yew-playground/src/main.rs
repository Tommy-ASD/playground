use chrono::{NaiveDateTime, Utc};
use yew::{
    function_component,
    prelude::{html, use_node_ref, Component, Context, Html},
    Callback, Hook, InputEvent, MouseEvent, NodeRef, Properties,
};

mod utilities;

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
    fn to_html(&self) -> (Html, Html) {
        (
            html! {
                <p>{ &self.added_at.to_string() }</p>
            },
            html! {
                <p>{ &self.text }</p>
            },
        )
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
                <p>{ &self.added_at.to_string() }</p>
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
    None,
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
            ChangeTodoList::None => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        utilities::set_cookie("test", "value");
        let link = ctx.link();

        let input_ref = NodeRef::default();

        let button_callback;
        {
            let input_ref = input_ref.clone();
            button_callback = link.callback(move |_event: MouseEvent| {
                let value = match input_ref.cast::<web_sys::HtmlInputElement>() {
                    Some(element) => element.value(),
                    None => return ChangeTodoList::None,
                };
                ChangeTodoList::AddItem(TodoItem::with_text(value))
            });
        };

        html! {
            <>
            <input
                id={"TodoListInput"}
                ref={input_ref}
            />
            <button
                onclick={button_callback}
            >
                { "Submit input" }
            </button>
            <table>
                <tr>
                    <th>{ "Name" }</th>
                    <th>{ "Created at" }</th>
                    <th>{ "Remove" }</th>
                </tr>
                { self.items.iter().enumerate().map(|(index, item)| {
                    let text = &item.text;
                    let added_at = &item.added_at;
                    html! {
                        <tr>
                            <th>{ text }</th>
                            <th>{ added_at.to_string() }</th>
                            <th>
                                <button
                                class={ "remove-todo-element-button" }
                                onclick={link.callback(move |_event: MouseEvent| {
                                    ChangeTodoList::RemoveItem(index)
                                })}/>
                            </th>
                        </tr>
                    }
                }).collect::<Vec<Html>>() }
            </table>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<TodoList>::new().render();
}
