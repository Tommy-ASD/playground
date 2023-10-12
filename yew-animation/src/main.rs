use std::collections::HashMap;

use plotters::prelude::IntoDrawingArea;
use uuid::Uuid;
use web_sys::HtmlCanvasElement;
use yew::{
    prelude::{function_component, html, Html},
    use_node_ref,
};
use yew_hooks::{use_raf, use_raf_state, UseRafStateHandle};

use graphlib::Graph;
use yew_canvas::inner::CanvasBackend;

use ordered_float::OrderedFloat;

#[function_component(CanvasRenderer)]
fn canvas_renderer() -> Html {
    let _ = use_raf(1000, 10); // Update the canvas on each requestAnimationFrame

    let reference = use_node_ref();

    let mut graph_state: UseRafStateHandle<Graph> = use_raf_state(|| Graph::new_random(25, 50));
    let mut node_positions_state = use_raf_state(|| graph_state.get_initial_positions());

    let canvas = reference.cast::<HtmlCanvasElement>();

    if let Some(canvas) = canvas {
        gloo::console::log!("Canvas exists");
        let mut node_positions = node_positions_state
            .iter()
            .map(|(key, (v1, v2))| (key.clone(), (v1.clone(), v2.clone())))
            .collect::<HashMap<Uuid, (OrderedFloat<f64>, OrderedFloat<f64>)>>();
        // let mut graph: Graph = *graph_state;
        graph_state.calculate_next_force_iteration(&mut node_positions, &mut OrderedFloat(1.0));
        // graph_state.apply_node_positions(&node_positions);
        node_positions_state.set(node_positions);
        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let root = backend.into_drawing_area();
        graph_state.draw_on_backend(&root);
        root.present();
    } else {
        gloo::console::log!("Canvas does not exist");
    }

    html! {
        <canvas id={"my-canvas"} width={1000} height={1000} ref={reference} />
    }
}

fn main() {
    yew::Renderer::<CanvasRenderer>::new().render();
}
