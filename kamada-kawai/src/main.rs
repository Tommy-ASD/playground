use std::collections::HashMap;

use drawing::{Drawing, DrawingIndex};
use ndarray::Array2;
use plotters::{
    prelude::{
        BitMapBackend, ChartBuilder, Circle, EmptyElement, IntoDrawingArea, LineSeries,
        PointSeries, RGBColor, BLACK, RED, WHITE, *,
    },
    style::text_anchor::{HPos, VPos},
};
use rand::Rng;
use shortest_path::all_sources_dijkstra;

mod drawing;
mod shortest_path;

fn norm(x: f32, y: f32) -> f32 {
    (x * x + y * y).sqrt().max(1.)
}

pub struct KamadaKawai {
    k_matrix: Array2<f32>,
    l_matrix: Array2<f32>,
    pub epsilon: f32,
}

impl KamadaKawai {
    pub fn new(graph: &Graph) -> KamadaKawai {
        let length_matrix = all_sources_dijkstra(graph);
        KamadaKawai::new_with_distance_matrix(&length_matrix)
    }

    pub fn new_with_distance_matrix(length_matrix: &Array2<f32>) -> KamadaKawai {
        let epsilon = 1e-1;
        let n = length_matrix.nrows();

        let mut stiffness_matrix = Array2::zeros((n, n));
        for i in 0..n {
            for j in 0..n {
                stiffness_matrix[[i, j]] = 1. / (length_matrix[[i, j]] * length_matrix[[i, j]]);
            }
        }
        KamadaKawai {
            k_matrix: stiffness_matrix,
            l_matrix: length_matrix.clone(),
            epsilon,
        }
    }

    pub fn select_node<N>(&self, drawing: &Drawing<N, f32>) -> Option<usize>
    where
        N: DrawingIndex,
    {
        let n = drawing.len();
        let KamadaKawai {
            k_matrix,
            l_matrix,
            epsilon,
            ..
        } = self;
        let mut max_delta_squared = 0.;
        let mut target_node = 0;
        for m in 0..n {
            let xm = drawing.coordinates[[m, 0]];
            let ym = drawing.coordinates[[m, 1]];
            let mut dedx = 0.;
            let mut dedy = 0.;
            for i in 0..n {
                if i != m {
                    let xi = drawing.coordinates[[i, 0]];
                    let yi = drawing.coordinates[[i, 1]];
                    let dx = xm - xi;
                    let dy = ym - yi;
                    let d = norm(dx, dy);
                    dedx += k_matrix[[m, i]] * (1. - l_matrix[[m, i]] / d) * dx;
                    dedy += k_matrix[[m, i]] * (1. - l_matrix[[m, i]] / d) * dy;
                }
            }
            let delta_squared = dedx * dedx + dedy * dedy;
            if delta_squared > max_delta_squared {
                max_delta_squared = delta_squared;
                target_node = m;
            }
        }

        if max_delta_squared < epsilon * epsilon {
            None
        } else {
            Some(target_node)
        }
    }

    pub fn apply_to_node<N>(&self, m: usize, drawing: &mut Drawing<N, f32>)
    where
        N: DrawingIndex,
    {
        let n = drawing.len();
        let KamadaKawai {
            k_matrix, l_matrix, ..
        } = self;
        let xm = drawing.coordinates[[m, 0]];
        let ym = drawing.coordinates[[m, 1]];
        let mut hxx = 0.;
        let mut hyy = 0.;
        let mut hxy = 0.;
        let mut dedx = 0.;
        let mut dedy = 0.;
        for i in 0..n {
            if i != m {
                let xi = drawing.coordinates[[i, 0]];
                let yi = drawing.coordinates[[i, 1]];
                let dx = xm - xi;
                let dy = ym - yi;
                let d = norm(dx, dy);
                let d_cubed = d * d * d;
                hxx += k_matrix[[m, i]] * (1. - l_matrix[[m, i]] * dy * dy / d_cubed);
                hyy += k_matrix[[m, i]] * (1. - l_matrix[[m, i]] * dx * dx / d_cubed);
                hxy += k_matrix[[m, i]] * l_matrix[[m, i]] * dx * dy / d_cubed;
                dedx += k_matrix[[m, i]] * (1. - l_matrix[[m, i]] / d) * dx;
                dedy += k_matrix[[m, i]] * (1. - l_matrix[[m, i]] / d) * dy;
            }
        }
        let determinant = hxx * hyy - hxy * hxy;
        let delta_x = (hyy * dedx - hxy * dedy) / determinant;
        let delta_y = (hxx * dedy - hxy * dedx) / determinant;
        drawing.coordinates[[m, 0]] -= delta_x;
        drawing.coordinates[[m, 1]] -= delta_y;
    }

    pub fn run<N>(&self, drawing: &mut Drawing<N, f32>)
    where
        N: DrawingIndex,
    {
        while let Some(m) = self.select_node(drawing) {
            self.apply_to_node(m, drawing);
        }
    }
}

fn visualize_kamada_kawai(kk: &KamadaKawai) -> Result<(), Box<dyn std::error::Error>> {
    // Create a new drawing area
    let root = BitMapBackend::new("kamada_kawai.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a new chart context
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-1.0..1.0, -1.0..1.0)?;

    // Draw nodes as circles and add labels
    for i in 0..kk.l_matrix.nrows() {
        let (x, y) = (
            kk.l_matrix[[i, 0]] as f64, // X coordinate
            kk.l_matrix[[i, 1]] as f64, // Y coordinate
        );
        println!("X: {x}, Y: {y}");

        chart.draw_series(std::iter::once(Circle::new(
            (x, y),
            5,
            ShapeStyle::from(&RED).filled(),
        )))?;

        // Adjust the label position relative to the node
        chart.draw_series(std::iter::once(Text::new(
            format!("{}", i),
            (x, y),
            ("sans-serif", 10).into_font().color(&RED),
        )))?;
    }

    // Draw lines representing k_matrix
    for i in 0..kk.l_matrix.nrows() {
        for j in 0..i {
            let x1 = kk.l_matrix[[i, 0]] as f64;
            let y1 = kk.l_matrix[[i, 1]] as f64;
            let x2 = kk.l_matrix[[j, 0]] as f64;
            let y2 = kk.l_matrix[[j, 1]] as f64;

            let weight = kk.k_matrix[[i, j]];

            let line = PathElement::new(
                vec![(x1, y1), (x2, y2)],
                ShapeStyle::from(&BLACK).stroke_width(weight as u32),
            );
            chart.draw_series(std::iter::once(line))?;
        }
    }

    // Display epsilon value
    chart.draw_series(std::iter::once(Text::new(
        format!("Epsilon: {:.5}", kk.epsilon),
        (-0.9, 0.9),
        ("sans-serif", 12).into_font().color(&BLACK),
    )))?;

    Ok(())
}

// #[test]
// fn test_kamada_kawai() {
//     use petgraph::Graph;

//     let n = 10;
//     let mut graph = Graph::new_undirected();
//     let nodes = (0..n).map(|_| graph.add_node(())).collect::<Vec<_>>();
//     for i in 0..n {
//         for j in 0..i {
//             graph.add_edge(nodes[j], nodes[i], ());
//         }
//     }

//     let mut coordinates = Drawing::initial_placement(&graph);

//     for &node in &nodes {
//         println!("{:?}", coordinates.position(node));
//     }

//     let kamada_kawai = KamadaKawai::new(&graph, &mut |_| 1.);
//     kamada_kawai.run(&mut coordinates);

//     for &node in &nodes {
//         println!("{:?}", coordinates.position(node));
//     }
// }

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let n = 10;
    let mut graph = Graph::new_random(1_000, 1_000);

    println!("Graph: {graph:?}");

    // visualize_graph(&mut graph);

    // let mut coordinates = Drawing::initial_placement(&graph);

    // for &node in &nodes {
    //     println!("{:?}", coordinates.position(node));
    // }

    let kamada_kawai = KamadaKawai::new(&graph);

    visualize_kamada_kawai(&kamada_kawai);

    // kamada_kawai.run(&mut coordinates);

    // for &node in &nodes {
    //     println!("{:?}", coordinates.position(node));
    // }
}

fn visualize_graph(graph: &mut Graph) -> Result<(), Box<dyn std::error::Error>> {
    let size = 10000;
    let mut rng = rand::thread_rng();
    // Create a drawing area
    let root = BitMapBackend::new("graph.png", (size as u32, size as u32)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .build_cartesian_2d(0..size, 0..size)?;

    let mut positions = HashMap::new();

    // Plot nodes as scatter points
    for node in &mut graph.nodes {
        let x = rng.gen_range(0..size);
        let y = rng.gen_range(0..size);

        positions.insert(node, (x, y));

        chart.draw_series(PointSeries::of_element(
            vec![(x, y)],
            5,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c) // We want the point to be at (x, y)
                        + Circle::new((0, 0), s, st.filled()); // And a circle that is 2 pixels large
            },
        ))?;
    }

    // Plot edges as lines
    // It is critical that this happens after the last loop, as the last loop defined positions
    for edge in &graph.edges {
        let source_pos = positions.get(&edge.0).unwrap();
        let target_pos = positions.get(&edge.1).unwrap();
        chart.draw_series(LineSeries::new(
            vec![source_pos.clone(), target_pos.clone()],
            &BLACK,
        ))?;
    }

    // Export the plot as an image
    root.present()?;
    Ok(())
}

/// A custom graph structure to represent a directed graph with nodes and weighted edges.
///
/// This struct provides a way to create, modify, and work with directed graphs where nodes are
/// represented by a generic type (typically numeric) and edges have associated weights.
///
/// # Examples
///
/// Creating a new graph:
///
/// ```rust
/// use my_graph_library::Graph;
///
/// let mut graph = Graph::new();
/// ```
///
/// Adding nodes and edges to the graph:
///
/// ```rust
/// # use my_graph_library::Graph;
/// let mut graph = Graph::new();
///
/// graph.add_node(0);
/// graph.add_node(1);
/// graph.add_edge(0, 1, 5.0);
/// ```
#[derive(Debug)]
pub struct Graph {
    /// The list of nodes in the graph.
    pub nodes: Vec<u32>,

    /// The list of weighted edges in the graph. Each edge is represented as a tuple containing
    /// the source node, target node, and edge weight.
    pub edges: Vec<(u32, u32, f32)>,
}

impl Graph {
    /// Creates a new, empty graph.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use my_graph_library::Graph;
    ///
    /// let mut graph = Graph::new();
    /// ```
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn new_random(num_nodes: u32, num_edges: u32) -> Self {
        let mut graph = Self::new();
        graph.randomize(num_nodes, num_edges);
        graph
    }

    fn randomize(&mut self, num_nodes: u32, num_edges: u32) {
        let mut rng = rand::thread_rng();

        // Generate random nodes
        for i in 0..num_nodes {
            self.add_node(i);
        }

        // Generate random edges
        for _ in 0..num_edges {
            let mut closure = || {
                let source_idx = rng.gen_range(0..self.nodes.len());
                let target_idx = rng.gen_range(0..self.nodes.len());

                let source_id = self.nodes[source_idx];
                let target_id = self.nodes[target_idx];

                // Add the edge
                self.add_edge(source_id, target_id, 1.0);
            };
            closure()
        }
    }

    /// Adds a node to the graph.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to be added to the graph.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use my_graph_library::Graph;
    /// let mut graph = Graph::new();
    ///
    /// graph.add_node(0);
    /// graph.add_node(1);
    /// ```
    pub fn add_node(&mut self, node: u32) {
        self.nodes.push(node);
    }

    /// Adds a weighted edge to the graph.
    ///
    /// # Arguments
    ///
    /// * `source` - The source node of the edge.
    /// * `target` - The target node of the edge.
    /// * `weight` - The weight of the edge.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use my_graph_library::Graph;
    /// let mut graph = Graph::new();
    ///
    /// graph.add_edge(0, 1, 5.0);
    /// ```
    pub fn add_edge(&mut self, source: u32, target: u32, weight: f32) {
        self.edges.push((source, target, weight));
    }
}
