extern crate petgraph;
extern crate plotters;

use std::sync::{Arc, Mutex};

use petgraph::dot::Dot;
use petgraph::graph::{DiGraph, NodeIndex};
use plotters::prelude::{
    BitMapBackend, ChartBuilder, Circle, EmptyElement, IntoDrawingArea, LineSeries, PointSeries,
    RGBColor, BLACK, RED, WHITE,
};

use rand::Rng;

#[derive(Debug)]
struct Node {
    position: (i32, i32),
}

impl Node {
    fn new_with_random_positions() -> Self {
        let mut rng = rand::thread_rng();
        Node {
            position: (rng.gen_range(0..=100), rng.gen_range(0..=100)),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a directed graph using petgraph
    let mut graph = DiGraph::new();

    let nodes = vec![
        Arc::new(Mutex::new(Node::new_with_random_positions())),
        Arc::new(Mutex::new(Node::new_with_random_positions())),
        Arc::new(Mutex::new(Node::new_with_random_positions())),
    ];

    // Add nodes to the graph and store their NodeIndex
    let a = graph.add_node(Arc::clone(&nodes[0]));
    let b = graph.add_node(Arc::clone(&nodes[1]));
    let c = graph.add_node(Arc::clone(&nodes[2]));

    println!("Generated {nodes:?}");

    // Add edges between nodes
    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, a, ());

    // Create a Plotters drawing area
    let root = BitMapBackend::new("graph.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .caption("Directed Graph Example", ("sans-serif", 40))
        .margin(10)
        .build_cartesian_2d(0..101, 0..101)?;

    // Customize the axes
    chart.configure_mesh().draw()?;

    // for node in graph.raw_nodes() {
    //     let inner = node.weight.as_ref().lock().unwrap();
    //     let pos = inner.position;
    //     chart.draw_series([Circle::new(pos, 5, RGBColor(255, 0, 0))])?;
    //     for i in 1..10 {
    //         let incoming = node.next_edge(petgraph::Direction::Incoming);
    //         let outgoing = node.next_edge(petgraph::Direction::Outgoing);

    //         println!("Incoming: {incoming:?}");
    //         println!("Outgoing: {outgoing:?}");
    //     }
    // }

    // Visualize the graph using Plotters
    for edge in graph.raw_edges() {
        let source_node = &graph[edge.source()].as_ref().lock().unwrap();
        let target_node = &graph[edge.target()].as_ref().lock().unwrap();

        let from = source_node.position;
        let to = target_node.position;

        chart.draw_series(LineSeries::new(vec![from, to], &BLACK))?;
        // chart.draw_series(PointSeries::of_element(
        //     vec![from, to],
        //     5,
        //     &RED,
        //     &|c, s, st| {
        //         return EmptyElement::at(c) // We want to fill it
        //             + Circle::new((0, 0), s, st.filled());
        //     },
        // ))?;
        chart.draw_series([Circle::new(from, 5, RGBColor(255, 0, 0))])?;
        chart.draw_series([Circle::new(to, 5, RGBColor(255, 0, 0))])?;
    }

    Ok(())
}
