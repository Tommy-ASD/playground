use graphlib::Graph;
use layout::Layout;
use plotters::prelude::{BitMapBackend, IntoDrawingArea, WHITE};

fn main() {
    let mut graph = Graph::new_random(1000, 2500);
    let root = BitMapBackend::new("before.png", (1000, 1000)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    graph.draw_on_backend(&root).unwrap();
    let layout = Layout::from_graph(&graph);
    layout.apply_to_graph(&mut graph);
    let root = BitMapBackend::new("after.png", (1000, 1000)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    graph.draw_on_backend(&root).unwrap();
}
