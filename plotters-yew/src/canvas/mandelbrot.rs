use crate::canvas::inner::CanvasBackend;
use crate::canvas::DrawResult;
use plotters::prelude::{ChartBuilder, HSLColor, IntoDrawingArea, BLACK, WHITE};
use std::ops::Range;
use web_sys::HtmlCanvasElement;

/// Draw Mandelbrot set
pub fn draw(
    element: HtmlCanvasElement,
    max_iterations: usize,
) -> DrawResult<impl Fn((i32, i32)) -> Option<(f64, f64)>> {
    let backend = CanvasBackend::with_canvas_object(element).unwrap();

    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_cartesian_2d(-2.1..0.6, -1.2..1.2)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let range = plotting_area.get_pixel_range();
    let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);
    let (xr, yr) = (chart.x_range(), chart.y_range());

    for (x, y, c) in generate_mandelbrot_set(xr, yr, (pw as usize, ph as usize), max_iterations) {
        if c != 100 {
            plotting_area.draw_pixel((x, y), &HSLColor(c as f64 / 100.0, 1.0, 0.5))?;
        } else {
            plotting_area.draw_pixel((x, y), &BLACK)?;
        }
    }

    root.present()?;
    return Ok(Box::new(chart.into_coord_trans()));
}

fn generate_mandelbrot_set(
    real_range: Range<f64>,
    complex_range: Range<f64>,
    sample_dimensions: (usize, usize),
    max_iterations: usize,
) -> impl Iterator<Item = (f64, f64, usize)> {
    let step_size = (
        (real_range.end - real_range.start) / sample_dimensions.0 as f64,
        (complex_range.end - complex_range.start) / sample_dimensions.1 as f64,
    );

    (0..(sample_dimensions.0 * sample_dimensions.1)).map(move |k| {
        let c = (
            real_range.start + step_size.0 * (k % sample_dimensions.0) as f64,
            complex_range.start + step_size.1 * (k / sample_dimensions.0) as f64,
        );

        let mut z = (0.0, 0.0);
        let mut iteration_count = 0;

        while iteration_count < max_iterations && z.0 * z.0 + z.1 * z.1 <= 1e10 {
            z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);
            iteration_count += 1;
        }

        (c.0, c.1, iteration_count)
    })
}
