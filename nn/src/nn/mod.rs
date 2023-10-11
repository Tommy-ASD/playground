use example_datasets::{first::first_dataset, Fruit};
use plotters::{
    backend::RGBPixel,
    coord::types::RangedCoordf64,
    prelude::{
        BitMapBackend, Cartesian2d, ChartBuilder, ChartContext, Circle, EmptyElement,
        IntoDrawingArea, PointSeries, RED, WHITE,
    },
    style::BLUE,
};
use serde::{Deserialize, Serialize};
use web_sys::HtmlCanvasElement;

use crate::canvas::CanvasBackend;

pub mod example_datasets;

#[derive(Serialize, Deserialize)]
struct Weights {
    weight11: f64,
    weight21: f64,
    weight12: f64,
    weight22: f64,
    bias1: f64,
    bias2: f64,
}

pub fn primary(element: HtmlCanvasElement) {
    let weights = Weights {
        weight11: 0.0,
        weight21: 0.0,
        weight12: 0.0,
        weight22: 0.0,
        bias1: 0.0,
        bias2: 0.0,
    };
    let root = CanvasBackend::with_canvas_object(element)
        .unwrap()
        .into_drawing_area();
    root.fill(&WHITE).unwrap();
    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .build_cartesian_2d(-1f64..1f64, -1f64..1f64)
        .unwrap();

    first_dataset()
        .into_iter()
        .for_each(|fruit| visualize(&fruit, &weights, &mut chart));

    root.present().unwrap();
}

fn classify(fruit: &Fruit, weights: &Weights) -> i64 {
    let out1 =
        fruit.spike_length * weights.weight11 + fruit.spot_size * weights.weight21 + weights.bias1;
    let out2: f64 =
        fruit.spike_length * weights.weight12 + fruit.spot_size * weights.weight22 + weights.bias2;

    return if out1 > out2 { 0 } else { 1 };
}

fn visualize(
    fruit: &Fruit,
    weights: &Weights,
    chart: &mut ChartContext<'_, CanvasBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
) {
    println!("Visualizing {fruit:?}");
    let predicted_class = classify(fruit, weights);

    if predicted_class == 0 {
        let point = PointSeries::of_element(
            vec![(fruit.spike_length, fruit.spot_size)],
            5,
            &BLUE,
            &|c, s, st| {
                return EmptyElement::at(c) // We want the point to be at (x, y)
                        + Circle::new((0, 0), s, st.filled()); // And a circle that is 2 pixels large
            },
        );
        chart.draw_series(point).unwrap();
    } else if predicted_class == 1 {
        let point = PointSeries::of_element(
            vec![(fruit.spike_length, fruit.spot_size)],
            5,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c) // We want the point to be at (x, y)
                        + Circle::new((0, 0), s, st.filled()); // And a circle that is 2 pixels large
            },
        );
        chart.draw_series(point).unwrap();
    }
}
