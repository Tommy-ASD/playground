use ws_client::get_ws_client;
use yew::{
    function_component,
    prelude::{html, Html},
};

use crate::{payload::PayloadList, state::State};

mod payload;
mod state;
mod utilities;
mod ws;
mod ws_client;

static BACKEND_URL: &str = "localhost:8081";

// Then supply the prop
#[function_component(App)]
fn app() -> Html {
    html! { <PayloadList /> }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
