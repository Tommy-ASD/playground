use crate::inner::CanvasBackend;
use crate::DrawResult;
use plotters::prelude::{ChartBuilder, FontDesc, IntoDrawingArea, LineSeries, RED, WHITE};
use web_sys::HtmlCanvasElement;

/// Draw power function f(x) = x^power.
pub fn draw(
    element: HtmlCanvasElement,
    power: i32,
) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let backend = CanvasBackend::with_canvas_object(element).unwrap();
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("y=x^{}", power), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(-1f32..1f32, -1.2f32..1.2f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    chart.draw_series(LineSeries::new(
        (-50..=50)
            .map(|x| x as f32 / 50.0)
            .map(|x| (x, x.powf(power as f32))),
        &RED,
    ))?;

    root.present()?;
    return Ok(chart.into_coord_trans());
}
