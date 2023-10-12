use yew::{
    prelude::{function_component, html, Html},
    use_state,
};
use yew_hooks::use_raf;

#[function_component(CanvasRenderer)]
fn canvas_renderer() -> Html {
    let _ = use_raf(1000, 100);
    let counter = use_state(|| 0);
    counter.set(*counter + 1);
    let val = *counter;

    gloo::console::log!("Rendering ", val);

    html! { <> { val } </> }
}

fn main() {
    yew::Renderer::<CanvasRenderer>::new().render();
}
