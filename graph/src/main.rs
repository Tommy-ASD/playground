use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use json::any_key_map;
use rand::Rng;

use uuid::Uuid;

use serde::{Deserialize, Serialize};

use plotters::{
    prelude::{
        BitMapBackend, ChartBuilder, Circle, EmptyElement, IntoDrawingArea, LineSeries,
        PointSeries, RGBColor, Text, BLACK, RED, WHITE,
    },
    style::IntoFont,
};

use ordered_float::OrderedFloat;

use crate::shortest_path::{bfs::dfs, dijkstra::dijkstra};

pub mod json;
pub mod shortest_path;

pub enum AddEdgeError {
    TargetNodeMissing(Uuid),
    SourceNodeMissing(Uuid),
    BothNodesMissing(Uuid, Uuid),
}

impl std::fmt::Display for AddEdgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AddEdgeError::TargetNodeMissing(_) => write!(f, "TargetNodeMissing"),
            AddEdgeError::SourceNodeMissing(_) => write!(f, "SourceNodeMissing"),
            AddEdgeError::BothNodesMissing(_, _) => write!(f, "BothNodesMissing"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeMetaData {
    pub position: (i32, i32),
}

impl NodeMetaData {
    pub fn new_random(range: Range<i32>) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(range.clone());
        let y = rng.gen_range(range.clone());

        Self { position: (x, y) }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeMetaData {
    pub weight: OrderedFloat<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge {
    pub incoming: Uuid,
    pub outgoing: Uuid,
    pub meta: EdgeMetaData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphNode {
    pub id: Uuid,
    pub meta: NodeMetaData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<GraphNode>,

    pub edges: Vec<Edge>,

    /// Mapping table from a node ID to the node's index in the Nodes vector
    pub node_lookup: HashMap<Uuid, usize>,
    #[serde(with = "any_key_map")]
    pub edge_lookup: HashMap<(Uuid, Uuid), usize>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            node_lookup: HashMap::new(),
            edge_lookup: HashMap::new(),
        }
    }

    fn add_node(&mut self) {
        let id = Uuid::new_v4();
        if !self.node_lookup.contains_key(&id) {
            let node = GraphNode {
                id,
                meta: NodeMetaData::new_random(1..10),
            };
            let index = self.nodes.len();
            self.nodes.push(node);
            self.node_lookup.insert(id, index);
        }
    }

    fn add_outgoing_edge(&mut self, source_id: Uuid, target_id: Uuid) -> Result<(), AddEdgeError> {
        match (
            self.node_lookup.get(&source_id),
            self.node_lookup.get(&target_id),
        ) {
            (Some(_), Some(_)) => {
                let edge = Edge {
                    incoming: source_id,
                    outgoing: target_id,
                    meta: EdgeMetaData {
                        weight: OrderedFloat(1.0),
                    },
                };
                self.edges.push(edge);
                self.edge_lookup
                    .insert((source_id, target_id), self.edges.len());
                Ok(())
            }
            (Some(_), None) => Err(AddEdgeError::TargetNodeMissing(target_id)),
            (None, Some(_)) => Err(AddEdgeError::SourceNodeMissing(source_id)),
            (None, None) => Err(AddEdgeError::BothNodesMissing(source_id, target_id)),
        }
    }

    fn add_incoming_edge(&mut self, source_id: Uuid, target_id: Uuid) -> Result<(), AddEdgeError> {
        self.add_outgoing_edge(target_id, source_id)
    }

    fn randomize(&mut self, num_nodes: usize, num_edges: usize) {
        let mut rng = rand::thread_rng();

        // Generate random nodes
        for _ in 0..num_nodes {
            self.add_node();
        }
        println!("Generated {num_nodes} nodes");

        // Generate random edges
        for _ in 0..num_edges {
            let source_idx = rng.gen_range(0..self.nodes.len());
            let target_idx = rng.gen_range(0..self.nodes.len());

            let source_id = self.nodes[source_idx].id;
            let target_id = self.nodes[target_idx].id;

            // Add the edge
            if let Err(err) = self.add_outgoing_edge(source_id, target_id) {
                eprintln!("Error adding edge: {}", err);
            }
        }
        println!("Generated {num_edges} edges");
    }

    fn new_random(num_nodes: usize, num_edges: usize) -> Self {
        let mut graph = Self::new();
        graph.randomize(num_nodes, num_edges);
        graph
    }

    fn get_random_node(&self) -> Uuid {
        let mut rng = rand::thread_rng();
        // Use the gen_range method to generate a random index
        let random_index = rng.gen_range(0..self.nodes.len());

        // Get the random element from the vector
        let random_element = self.nodes[random_index].id;
        random_element
    }
    fn get_neighbors(&self, node_id: Uuid) -> HashSet<Uuid> {
        let mut neighbors = HashSet::new();

        // Find the index of the node with the given ID in the node vector
        if let Some(&node_index) = self.node_lookup.get(&node_id) {
            // Iterate through the edges to find outgoing edges from the node
            for edge in &self.edges {
                if edge.incoming == node_id {
                    // If the incoming ID matches the target node, add the outgoing node to neighbors
                    neighbors.insert(edge.outgoing);
                }
                if edge.outgoing == node_id {
                    // If the incoming ID matches the target node, add the outgoing node to neighbors
                    neighbors.insert(edge.incoming);
                }
            }
        }

        neighbors
    }
}

fn main() {
    let mut graph = Graph::new_random(10, 15);
    let _ = visualize_graph(&mut graph);
    for _ in 0..10 {
        let source_id = graph.get_random_node();
        let target_id = graph.get_random_node();

        println!("Running Shortest Path for {source_id}, {target_id}");

        let shortest = dfs(&graph, source_id);

        println!("Shortest Path result: {shortest:?}");
        println!(
            "Shortest Path result: {shortest_json}",
            shortest_json = serde_json::to_string_pretty(&shortest).unwrap()
        );
    }

    // println!("Dijkstra: {dijkstra:?}");

    // println!("Graph: {graph:#?}");

    println!(
        "{graph_json}",
        graph_json = serde_json::to_string_pretty(&graph).unwrap()
    );
}

fn visualize_graph(graph: &mut Graph) -> Result<(), Box<dyn std::error::Error>> {
    let size = 1000;
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

    // Plot nodes as scatter points
    for node in &mut graph.nodes {
        let x = rng.gen_range(0..size);
        let y = rng.gen_range(0..size);

        node.meta = NodeMetaData { position: (x, y) };

        chart.draw_series(PointSeries::of_element(
            vec![(x, y)],
            5,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c) // We want the point to be at (x, y)
                        + Circle::new((0, 0), s, st.filled()) // And a circle that is 2 pixels large
                        + Text::new(
                        format!("{}", node.id), // Convert the UUID to a string and display it
                        (0, 0), // Adjust the position to display below the point
                        ("sans-serif", 25.0).into_font(),
                    ); // Add text below the point
            },
        ))?;
    }

    for edge in &graph.edges {
        let source_pos = graph
            .nodes
            .get(*graph.node_lookup.get(&edge.incoming).unwrap())
            .unwrap()
            .meta
            .position;
        let target_pos = graph
            .nodes
            .get(*graph.node_lookup.get(&edge.outgoing).unwrap())
            .unwrap()
            .meta
            .position;
        chart.draw_series(LineSeries::new(vec![source_pos, target_pos], &BLACK))?;
    }
    // Export the plot as an image
    root.present()?;
    Ok(())
}
