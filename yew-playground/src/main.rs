use yew::prelude::{html, Component, Context, Html};

enum Msg {
    AddElement(String),
}

struct ListComponent {
    elements: Vec<String>,
}

impl Component for ListComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { elements: vec![] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddElement(element) => {
                self.elements.push(element);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <ul>
                {
                    self.elements.iter().map(|el| html!{
                        <li> { el } </li>
                    }).collect::<Vec<Html>>()
                }
                </ul>
                <input onclick={link.callback(|_| Msg::AddElement("Test".to_string()))}/>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<ListComponent>::new().render();
}
