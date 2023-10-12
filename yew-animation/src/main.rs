use std::collections::HashMap;

use plotters::{prelude::IntoDrawingArea, style::WHITE};
use uuid::Uuid;
use web_sys::{HtmlCanvasElement, HtmlInputElement};
use yew::{
    prelude::{function_component, html, Html},
    use_node_ref, use_state, Callback, NodeRef,
};
use yew_hooks::{use_raf, use_raf_state, UseRafStateHandle};

use graphlib::Graph;
use yew_canvas::CanvasBackend;

use ordered_float::OrderedFloat;

use crate::slider::Slider;

mod slider;

#[function_component(CanvasRenderer)]
fn canvas_renderer() -> Html {
    let _ = use_raf(1000, 10); // Update the canvas on each requestAnimationFrame

    let rep_strength = use_state(|| 0.001);
    let spr_stiff = use_state(|| 0.0005);

    let reference = use_node_ref();

    let graph_state: UseRafStateHandle<Graph> = use_raf_state(|| {
        let val = Graph::new_random(25, 50);
        val
    });
    let node_positions_state = use_raf_state(|| {
        let val = graph_state.get_initial_positions();
        val
    });

    let canvas = reference.cast::<HtmlCanvasElement>();

    if let Some(canvas) = canvas {
        let (rep_strength, spr_stiff) = (rep_strength.clone(), spr_stiff.clone());
        let mut node_positions = node_positions_state
            .iter()
            .map(|(key, (v1, v2))| (key.clone(), (v1.clone(), v2.clone())))
            .collect::<HashMap<Uuid, (OrderedFloat<f64>, OrderedFloat<f64>)>>();
        let mut graph: Graph = Graph {
            nodes: graph_state.nodes.clone(),
            edges: graph_state.edges.clone(),
            node_lookup: graph_state.node_lookup.clone(),
            edge_lookup: graph_state.edge_lookup.clone(),
        };
        // gloo::console::log!(
        //     "Node positions before: ",
        //     serde_json::to_string_pretty(&node_positions).unwrap()
        // );
        graph.calculate_next_force_iteration(
            &mut node_positions,
            OrderedFloat(*rep_strength),
            OrderedFloat(*spr_stiff),
            &mut OrderedFloat(1.0),
        );
        // gloo::console::log!(
        //     "Node positions after: ",
        //     serde_json::to_string_pretty(&node_positions).unwrap()
        // );
        graph.apply_node_positions(&node_positions);
        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let root = backend.into_drawing_area();
        root.fill(&WHITE).unwrap();
        graph.draw_on_backend(&root).unwrap();
        root.present().unwrap();
        node_positions_state.set(node_positions);
        graph_state.set(graph);
    }

    html! {
        <>
            <canvas id={"my-canvas"} width={600} height={600} ref={reference} />
            <Slider label="Spring stiffness"
                min=0.0 max=1.0
                onchange={{
                    let spr_stiff = spr_stiff.clone();
                    Callback::from(move |val: f64| {
                        gloo::console::log!(val);
                        spr_stiff.set(val);
                    })
                }}
                value={*spr_stiff}
                step=0.00001
                precision=5
            />
            <br />
            <Slider label="Repulsion strength"
                min=0.0 max=1.0
                onchange={{
                    let rep_strength = rep_strength.clone();
                    Callback::from(move |val: f64| {
                        gloo::console::log!(val);
                        rep_strength.set(val);
                    })
                }}
                value={*rep_strength}
                step=0.00001
                precision=5
            />
        </>
    }
}

fn main() {
    yew::Renderer::<CanvasRenderer>::new().render();
}
