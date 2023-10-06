use std::collections::{HashMap, HashSet};

use rand::Rng;

use uuid::Uuid;

use serde::{Deserialize, Serialize};

use plotters::prelude::{
    BitMapBackend, ChartBuilder, Circle, EmptyElement, IntoDrawingArea, LineSeries, PointSeries,
    RGBColor, BLACK, RED, WHITE,
};

enum AddEdgeError {
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

#[derive(Debug, Serialize, Deserialize)]
struct NodeMetaData {
    position: (i32, i32),
}

#[derive(Debug, Serialize, Deserialize)]
struct GraphNode {
    pub outgoing: HashSet<Uuid>,
    pub incoming: HashSet<Uuid>,
    pub id: Uuid,
    meta: Option<NodeMetaData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Graph {
    pub nodes: Vec<GraphNode>,

    /// Mapping table from a node ID to the node's index in the Nodes vector
    pub node_lookup: HashMap<Uuid, usize>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            node_lookup: HashMap::new(),
        }
    }

    fn add_node(&mut self) {
        let id = Uuid::new_v4();
        if !self.node_lookup.contains_key(&id) {
            let node = GraphNode {
                outgoing: HashSet::new(),
                incoming: HashSet::new(),
                id,
                meta: None,
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
            (Some(source_idx), Some(target_idx)) => {
                self.nodes[*source_idx].outgoing.insert(target_id);
                self.nodes[*target_idx].incoming.insert(source_id);
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

        // Generate random edges
        for _ in 0..num_edges {
            let mut closure = || {
                let source_idx = rng.gen_range(0..self.nodes.len());
                let target_idx = rng.gen_range(0..self.nodes.len());

                let source_id = self.nodes[source_idx].id;
                let target_id = self.nodes[target_idx].id;

                // Add the edge
                if let Err(err) = self.add_outgoing_edge(source_id, target_id) {
                    eprintln!("Error adding edge: {}", err);
                }
            };
            closure()
        }
    }

    fn new_random(num_nodes: usize, num_edges: usize) -> Self {
        let mut graph = Self::new();
        graph.randomize(num_nodes, num_edges);
        graph
    }
}

fn main() {
    let mut graph = Graph::new_random(10, 10);

    println!(
        "{graph_json}",
        graph_json = serde_json::to_string_pretty(&graph).unwrap()
    );

    visualize_graph(&mut graph);
}

fn visualize_graph(graph: &mut Graph) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    // Create a drawing area
    let root = BitMapBackend::new("graph.png", (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .build_cartesian_2d(0..1000, 0..1000)?;

    // Plot nodes as scatter points
    for node in &mut graph.nodes {
        let x = rng.gen_range(0..1000);
        let y = rng.gen_range(0..1000);

        node.meta = Some(NodeMetaData { position: (x, y) });

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
    for node in &graph.nodes {
        let source_pos = node.meta.as_ref().unwrap().position;
        for &outgoing_id in &node.outgoing {
            let target_idx = graph.node_lookup.get(&outgoing_id).unwrap();
            let target_node = graph.nodes.get(*target_idx).unwrap();
            let target_pos = target_node.meta.as_ref().unwrap().position;

            chart.draw_series(LineSeries::new(vec![source_pos, target_pos], &BLACK))?;
        }
    }

    // Export the plot as an image
    root.present()?;
    Ok(())
}
