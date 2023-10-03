mod inner;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

pub mod func_plot;
pub mod mandelbrot;
pub mod plot3d;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

/// Result of screen to chart coordinates conversion.
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Chart {
    /// Draw provided power function on the canvas element using it's id.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn power(element: HtmlCanvasElement, power: i32) -> Result<Chart, JsValue> {
        let map_coord = func_plot::draw(element, power).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    /// Draw Mandelbrot set on the provided canvas element.
    /// Return `Chart` struct suitable for coordinate conversion.
    pub fn mandelbrot(canvas: HtmlCanvasElement, max_iterations: usize) -> Result<Chart, JsValue> {
        let map_coord = mandelbrot::draw(canvas, max_iterations).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(map_coord),
        })
    }

    pub fn plot3d(canvas: HtmlCanvasElement, pitch: f64, yaw: f64) -> Result<(), JsValue> {
        plot3d::draw(canvas, pitch, yaw).map_err(|err| err.to_string())?;
        Ok(())
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<Point> {
        (self.convert)((x, y)).map(|(x, y)| Point { x, y })
    }
}
