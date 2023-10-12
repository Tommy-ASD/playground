use std::collections::HashMap;

use plotters::{prelude::IntoDrawingArea, style::WHITE};
use uuid::Uuid;
use web_sys::{HtmlCanvasElement, HtmlInputElement};
use yew::{
    prelude::{function_component, html, Html},
    use_node_ref, Callback, NodeRef,
};
use yew_hooks::{use_raf, use_raf_state, UseRafStateHandle};

use graphlib::Graph;
use yew_canvas::CanvasBackend;

use ordered_float::OrderedFloat;

mod slider;

thread_local! {
    pub static REPULSION_STRENGTH_REF: NodeRef = NodeRef::default();
    pub static SPRING_STIFFNESS_REF: NodeRef = NodeRef::default();
    pub static REPULSION_STRENGTH_LABEL_REF: NodeRef = NodeRef::default();
    pub static SPRING_STIFFNESS_LABEL_REF: NodeRef = NodeRef::default();
    pub static REPULSION_STRENGTH: OrderedFloat<f64> = OrderedFloat(0.01);
    pub static SPRING_STIFFNESS: OrderedFloat<f64> = OrderedFloat(0.01);
}

#[derive(Clone)]
pub struct State {
    repulsion_strength_ref: NodeRef,
    spring_stiffness_ref: NodeRef,
    repulsion_strength_label_ref: NodeRef,
    spring_stiffness_label_ref: NodeRef,
    repulsion_strength: OrderedFloat<f64>,
    spring_stiffness: OrderedFloat<f64>,
}

impl State {
    fn get() -> Self {
        Self {
            repulsion_strength_ref: REPULSION_STRENGTH_REF.with(|inner| inner.clone()),
            spring_stiffness_ref: SPRING_STIFFNESS_REF.with(|inner| inner.clone()),
            repulsion_strength_label_ref: REPULSION_STRENGTH_LABEL_REF.with(|inner| inner.clone()),
            spring_stiffness_label_ref: SPRING_STIFFNESS_LABEL_REF.with(|inner| inner.clone()),
            repulsion_strength: REPULSION_STRENGTH.with(|inner| inner.clone()),
            spring_stiffness: SPRING_STIFFNESS.with(|inner| inner.clone()),
        }
    }
}

#[function_component(CanvasRenderer)]
fn canvas_renderer() -> Html {
    let _ = use_raf(1000, 10); // Update the canvas on each requestAnimationFrame

    let reference = use_node_ref();

    let graph_state: UseRafStateHandle<Graph> = use_raf_state(|| {
        let val = Graph::new_random(25, 50);
        gloo::console::log!("Initializing graph");
        val
    });
    let node_positions_state = use_raf_state(|| {
        let val = graph_state.get_initial_positions();
        gloo::console::log!("Initializing node positions");
        val
    });

    let canvas = reference.cast::<HtmlCanvasElement>();

    if let Some(canvas) = canvas {
        gloo::console::log!("Canvas exists");
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
        graph.calculate_next_force_iteration(&mut node_positions, &mut OrderedFloat(1.0));
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
    } else {
        gloo::console::log!("Canvas does not exist");
    }

    let state = State::get();

    let rep_strength_input_callback = {
        let rep_strength_ref = state.repulsion_strength_ref.clone();
        Callback::from(move |ev| {
            gloo::console::log!("Got event ", ev);
            let el = rep_strength_ref.cast::<HtmlInputElement>().unwrap();
            let value = el.value_as_number();
            gloo::console::log!("Element: ", el);
            gloo::console::log!("Value: ", value);
        })
    };

    html! {
        <>
            <canvas id={"my-canvas"} width={600} height={600} ref={reference} />
            <label for={"spring-stiffness"} ref={state.spring_stiffness_label_ref}>
                { &format!("Spring stiffness ({spr_stiff}): ", spr_stiff = state.spring_stiffness) }
            </label>
            <input
                type={"range"}
                id={"spring-stiffness"}
                ref={state.spring_stiffness_ref}
                min={ "0.1 "}
                max={ "3.0 "}
                step={ "0.1 "}
                // value={ format!("{}", state.spring_stiffness) }
            />
            <br />
            <label for={"repulsion-strength"} ref={state.repulsion_strength_label_ref}>
                { &format!("Repulsion strength ({rep_strength}): ", rep_strength = state.repulsion_strength) }
            </label>
            <input
                type={"range"}
                id={"repulsion-strength"}
                ref={state.repulsion_strength_ref}
                min={ "0.1 "}
                max={ "3.0 "}
                step={ "0.001 "}
                oninput={rep_strength_input_callback}
                // value={ format!("{}", state.repulsion_strength) }
            />
        </>
    }
}

fn main() {
    yew::Renderer::<CanvasRenderer>::new().render();
}
