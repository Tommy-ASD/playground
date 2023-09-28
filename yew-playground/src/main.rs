use std::fmt::Display;

use strum::{EnumIter, EnumString, IntoEnumIterator};
use yew::prelude::{function_component, html, Callback, Component, Context, Html};
use yew_router::prelude::{use_location, use_navigator, BrowserRouter, Link, Routable, Switch};

enum Msg {
    AddElement(String),
}

#[derive(Clone)]
struct ListComponent {
    elements: Vec<String>,
}

impl ListComponent {
    fn new() -> Self {
        Self { elements: vec![] }
    }
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

#[derive(Clone, Routable, PartialEq, EnumIter, EnumString, Debug)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[at("/news/:id")]
    News { id: u8 },
    #[at("/list")]
    List,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Route::Home => write!(f, "Home"),
            &Route::Secure => write!(f, "Secure"),
            &Route::News { id: _ } => write!(f, "News"),
            &Route::List => write!(f, "List"),
            &Route::NotFound => write!(f, "404 Not Found"),
        }
    }
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    let location = use_location();
    let router_elements = Route::iter()
        .map(|variant| {
            html! {
                <li>
                    <Link<Route> to={variant.clone()}>
                        { format!("{}", variant) }
                    </Link<Route>>
                </li>
            }
        })
        .collect::<Vec<Html>>();
    match routes {
        Route::Home => {
            html! { <ul>{ router_elements }</ul> }
        }
        Route::Secure => html! {
            <Secure />
        },
        Route::News { id } => html! { <h1>{ id }</h1> },
        Route::List => html! {<ListComponent/>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[derive(Clone)]
struct Main {}

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={|a| {
                    switch(a)
                }} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        }
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
