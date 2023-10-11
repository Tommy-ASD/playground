use canvas::inner::CanvasBackend;
use graph::Graph;
use plotters::{
    coord::Shift,
    prelude::{DrawingArea, IntoDrawingArea},
};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};
use web_sys::{
    HtmlCanvasElement, HtmlDivElement, HtmlInputElement, HtmlLabelElement, HtmlSelectElement,
};
use yew::{html, Callback, Component, Context, Html, NodeRef};

use crate::canvas::{func_plot, mandelbrot, plot3d};

mod canvas;
mod graph;

pub struct App;

thread_local! {
    pub static SELECT_REF: NodeRef = NodeRef::default();
    pub static CANVAS_REF: NodeRef = NodeRef::default();
    pub static THREED_CTRLS_REF: NodeRef = NodeRef::default();
    pub static PITCH_REF: NodeRef = NodeRef::default();
    pub static YAW_REF: NodeRef = NodeRef::default();
    pub static MANDELBROT_CTRLS_REF: NodeRef = NodeRef::default();
    pub static ITERS_REF: NodeRef = NodeRef::default();
    pub static ITERS_LABEL_REF: NodeRef = NodeRef::default();
    pub static STATUS_REF: NodeRef = NodeRef::default();

    pub static GRAPH: Graph = Graph::new_random(25, 50);
    pub static INITIALIZED_CANVAS: bool = false;
}

enum UpgradeType {
    Graph(i32),
    Mandelbrot(usize),
    Plot3d(f64, f64),
}

#[derive(Clone)]
pub struct State {
    select_ref: NodeRef,
    canvas_ref: NodeRef,
    threed_ctrls_ref: NodeRef,
    pitch_ref: NodeRef,
    yaw_ref: NodeRef,
    mandelbrot_ctrls_ref: NodeRef,
    iters_ref: NodeRef,
    iters_label_ref: NodeRef,
    status_ref: NodeRef,
    graph: Graph,
    initialized_canvas: bool,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        let State {
            select_ref: _,
            canvas_ref: _,
            threed_ctrls_ref: _,
            pitch_ref: _,
            yaw_ref: _,
            mandelbrot_ctrls_ref: _,
            iters_ref: _,
            iters_label_ref: _,
            status_ref: _,
            graph: _,
            mut initialized_canvas,
        } = get_state();
        if !initialized_canvas {
            update_plot();
            initialized_canvas = true;
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = get_state();

        let callback = ctx.link().callback(move |_| ());

        html! {
            <>
                <main>
                    <h1>{"Plotters and Yew Demo"}</h1>
                    <div id={"coord"}></div>
                    <canvas id={"canvas"} width={"1000"} height={"1000"} ref={state.canvas_ref}></canvas>
                    <div id={"status"} ref={state.status_ref}>{"Loading Plotters..."}</div>
                    <div id={"control"}>
                        <label for={"plot-type"}>{"Demo: "}</label>
                    </div>
                    <button onclick={callback}>{ "Initialize the thingy" }</button>
                </main>
            </>
        }
    }
}

fn update_plot() {
    let mut state = get_state();
    gloo::console::log!("Updating plot");
    state.graph.full_fdl(1000);
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn get_state() -> State {
    State {
        select_ref: CANVAS_REF.with(|inner| inner.clone()),
        canvas_ref: SELECT_REF.with(|inner| inner.clone()),
        threed_ctrls_ref: THREED_CTRLS_REF.with(|inner| inner.clone()),
        pitch_ref: PITCH_REF.with(|inner| inner.clone()),
        yaw_ref: YAW_REF.with(|inner| inner.clone()),
        mandelbrot_ctrls_ref: MANDELBROT_CTRLS_REF.with(|inner| inner.clone()),
        iters_ref: ITERS_REF.with(|inner| inner.clone()),
        iters_label_ref: ITERS_LABEL_REF.with(|inner| inner.clone()),
        status_ref: STATUS_REF.with(|inner| inner.clone()),
        graph: GRAPH.with(|inner| inner.clone()),
        initialized_canvas: INITIALIZED_CANVAS.with(|inner| inner.clone()),
    }
}

fn get_canvas() -> Option<DrawingArea<CanvasBackend, Shift>> {
    let State {
        select_ref: _,
        canvas_ref,
        threed_ctrls_ref: _,
        pitch_ref: _,
        yaw_ref: _,
        mandelbrot_ctrls_ref: _,
        iters_ref: _,
        iters_label_ref: _,
        status_ref: _,
        graph: _,
        initialized_canvas: _,
    } = get_state();
    gloo::console::log!("7");

    let canvas = match canvas_ref.cast::<HtmlCanvasElement>() {
        Some(element) => element,
        None => {
            gloo::console::log!("No input was provided");
            return None;
        }
    };

    gloo::console::log!("8");

    let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
    let root = backend.into_drawing_area();
    gloo::console::log!("9");
    Some(root)
}

// JavaScript interop to call the requestAnimationFrameCallback function
fn request_animation_frame_callback(window: &web_sys::Window, callback: Callback<()>) {
    let callback = move || {
        callback.emit(());
    };
    let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);
    window
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame`");
    callback.forget();
}
