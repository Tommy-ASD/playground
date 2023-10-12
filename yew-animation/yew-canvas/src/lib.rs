pub mod inner;

pub mod func_plot;
pub mod mandelbrot;
pub mod plot3d;

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;
