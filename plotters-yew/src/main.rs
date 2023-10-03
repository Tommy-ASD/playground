use std::sync::Mutex;

use web_sys::{Event, HtmlDivElement, HtmlInputElement, HtmlLabelElement, MouseEvent};
use yew::{html, Callback, Component, Context, Html, NodeRef};

use crate::canvas::{func_plot, mandelbrot, plot3d, Chart};

use once_cell::sync::Lazy;

mod canvas;
mod types;

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
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let state = get_state();

        html! {
            <>
                <main>
                    <h1>{"Plotters and Yew Demo"}</h1>
                    <div id={"coord"}></div>
                    <canvas id={"canvas"} width={"600"} height={"400"} ref={state.canvas_ref}></canvas>
                    <div id={"status"}>{"Loading Plotters..."}</div>
                    <div id={"control"}>
                        <label for={"plot-type"}>{"Demo: "}</label>
                        <select id={"plot-type"} ref={state.select_ref} onchange={selection_callback}>
                            <option value={"0"}>{"Graph of y=1"}</option>
                            <option value={"1"}>{"Graph of y=x"}</option>
                            <option value={"2"}>{"Graph of y=x^2"}</option>
                            <option value={"3"}>{"Graph of y=x^3"}</option>
                            <option value={"4"}>{"Graph of y=x^4"}</option>
                            <option value={"5"}>{"Graph of y=x^5"}</option>
                            <option value={"mandelbrot"}>{"Mandelbrot Set"}</option>
                            <option value="3d-plot">{"3D Plot Demo"}</option>
                        </select>
                        <div id={"3d-control"} hidden={true} ref={state.threed_ctrls_ref}>
                            <label for={"pitch"}>{"Pitch: "}</label>
                            <input type={"range"} min={"0"} max={"157"} id={"pitch"} ref={state.pitch_ref} onchange={update_plot_3d} /> <br />
                            <label for={"yaw"}>{"Yaw: "}</label>
                            <input type={"range"} min={"0"} max={"314"} id={"yaw"} ref={state.yaw_ref} onchange={update_plot_3d} />
                        </div>
                        <div id={"mandelbrot-control"} hidden={true} ref={state.mandelbrot_ctrls_ref}>
                            <label for={"iterations"} ref={state.iters_label_ref}>{"Iterations (50): "}</label>
                            <input type={"number"} min={"0"} id={"iterations"} ref={state.iters_ref} onchange={update_mandelbrot_iterators} /> <br />
                        </div>
                    </div>
                </main>
                <footer>
                    <a href={"https://github.com/plotters-rs/plotters-wasm-demo"} target={"a"}>{"Source"}</a> { " | " }
                    <a href={"https://github.com/plotters-rs/plotters"} target={"a"}>{"Repo"}</a> { " | " }
                    <a href={"https://crates.io/crates/plotters"} target={"a"}>{"Crates"}</a> { " | " }
                    <a href={"https://docs.rs/plotters"} target={"a"}>{"Docs"}</a> { " | " }
                </footer>
            </>
        }
    }
}

fn update_mandelbrot_iterators<T>(_: T) {
    let state = get_state();
    // handle label first
    let label = match state.iters_label_ref.cast::<HtmlLabelElement>() {
        Some(element) => element,
        None => {
            gloo::console::log!("No input was provided");
            return;
        }
    };

    let canvas = match state.canvas_ref.cast::<web_sys::HtmlCanvasElement>() {
        Some(element) => element,
        None => {
            gloo::console::log!("No input was provided");
            return;
        }
    };
    let iters = match state.iters_ref.cast::<web_sys::HtmlInputElement>() {
        Some(element) => element.value_as_number(),
        None => {
            gloo::console::log!("No input was provided");
            return;
        }
    };
    label.set_inner_text(&format!("Iterations: {}", iters as usize));
    let _ = mandelbrot::draw(canvas, iters as usize);
}

fn selection_callback<T>(_: T) {
    let state = get_state();
    let select = match state.select_ref.cast::<web_sys::HtmlSelectElement>() {
        Some(element) => element,
        None => {
            gloo::console::log!("No input was provided");
            return;
        }
    };
    let element = match state.canvas_ref.cast::<web_sys::HtmlCanvasElement>() {
        Some(element) => element,
        None => {
            gloo::console::log!("No input was provided");
            return;
        }
    };
    let key = select.selected_index();
    let threed_controls: HtmlDivElement =
        match state.threed_ctrls_ref.cast::<web_sys::HtmlDivElement>() {
            Some(element) => element,
            None => {
                gloo::console::log!("No input was provided");
                return;
            }
        };
    threed_controls.set_hidden(true);
    let mandelbrot_controls: HtmlDivElement =
        match state.mandelbrot_ctrls_ref.cast::<web_sys::HtmlDivElement>() {
            Some(element) => element,
            None => {
                gloo::console::log!("No input was provided");
                return;
            }
        };
    mandelbrot_controls.set_hidden(true);

    match key {
        0 | 1 | 2 | 3 | 4 | 5 => {
            let _ = func_plot::draw(element, key);
            return;
        }
        6 => {
            mandelbrot_controls.set_hidden(false);
            let iters = match state.iters_ref.cast::<web_sys::HtmlInputElement>() {
                Some(element) => element.value_as_number(),
                None => {
                    gloo::console::log!("No input was provided");
                    return;
                }
            };
            let _ = mandelbrot::draw(element, iters as usize);
            return;
        }
        7 => {
            threed_controls.set_hidden(false);

            let pitch = match state.pitch_ref.cast::<web_sys::HtmlInputElement>() {
                Some(element) => element.value_as_number(),
                None => {
                    gloo::console::log!("No input was provided");
                    return;
                }
            };
            let yaw = match state.yaw_ref.cast::<web_sys::HtmlInputElement>() {
                Some(element) => element.value_as_number(),
                None => {
                    gloo::console::log!("No input was provided");
                    return;
                }
            };
            let _ = plot3d::draw(element, pitch, yaw);
            return;
        }
        _ => {}
    }
}

fn update_plot_3d<T>(_: T) {
    let state = get_state();

    let canvas = match state.canvas_ref.cast::<web_sys::HtmlCanvasElement>() {
        Some(element) => element,
        None => {
            gloo::console::log!("No input was provided");
            return;
        }
    };
    let pitch = match state.pitch_ref.cast::<web_sys::HtmlInputElement>() {
        Some(element) => element.value_as_number(),
        None => {
            gloo::console::log!("No input was provided");
            return;
        }
    };
    let yaw = match state.yaw_ref.cast::<web_sys::HtmlInputElement>() {
        Some(element) => element.value_as_number(),
        None => {
            gloo::console::log!("No input was provided");
            return;
        }
    };
    let _ = plot3d::draw(canvas, pitch, yaw);
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn get_state() -> State {
    let (
        select_ref,
        canvas_ref,
        threed_ctrls_ref,
        pitch_ref,
        yaw_ref,
        mandelbrot_ctrls_ref,
        iters_ref,
        iters_label_ref,
    ) = (
        CANVAS_REF.with(|inner| inner.clone()),
        SELECT_REF.with(|inner| inner.clone()),
        THREED_CTRLS_REF.with(|inner| inner.clone()),
        PITCH_REF.with(|inner| inner.clone()),
        YAW_REF.with(|inner| inner.clone()),
        MANDELBROT_CTRLS_REF.with(|inner| inner.clone()),
        ITERS_REF.with(|inner| inner.clone()),
        ITERS_LABEL_REF.with(|inner| inner.clone()),
    );
    State {
        select_ref,
        canvas_ref,
        threed_ctrls_ref,
        pitch_ref,
        yaw_ref,
        mandelbrot_ctrls_ref,
        iters_ref,
        iters_label_ref,
    }
}
