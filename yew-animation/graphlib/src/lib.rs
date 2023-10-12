use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use json::any_key_map;
use meta::{Coordinate, EdgeMetaData, Line, NodeMetaData};
use rand::Rng;

use uuid::Uuid;

use serde::{Deserialize, Serialize};

use ordered_float::OrderedFloat;

pub mod force;
pub mod json;
// pub mod math;
pub mod meta;
pub mod shortest_path;
pub mod visualize;

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Edge {
    pub incoming: Uuid,
    pub outgoing: Uuid,
    pub meta: EdgeMetaData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct GraphNode {
    pub id: Uuid,
    pub meta: NodeMetaData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Graph {
    pub nodes: Vec<GraphNode>,

    pub edges: Vec<Edge>,

    /// Mapping table from a node ID to the node's index in the Nodes vector
    pub node_lookup: HashMap<Uuid, usize>,
    #[serde(with = "any_key_map")]
    pub edge_lookup: HashMap<(Uuid, Uuid), usize>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            node_lookup: HashMap::new(),
            edge_lookup: HashMap::new(),
        }
    }

    pub fn add_node(&mut self) {
        let id = Uuid::new_v4();
        if !self.node_lookup.contains_key(&id) {
            let node = GraphNode {
                id,
                meta: NodeMetaData::new_random(OrderedFloat(-1f64)..OrderedFloat(1f64)),
            };
            let index = self.nodes.len();
            self.nodes.push(node);
            self.node_lookup.insert(id, index);
        }
    }

    pub fn add_outgoing_edge(
        &mut self,
        source_id: Uuid,
        target_id: Uuid,
    ) -> Result<(), AddEdgeError> {
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

    pub fn add_incoming_edge(
        &mut self,
        source_id: Uuid,
        target_id: Uuid,
    ) -> Result<(), AddEdgeError> {
        self.add_outgoing_edge(target_id, source_id)
    }

    pub fn randomize(&mut self, num_nodes: usize, num_edges: usize) {
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

    pub fn new_random(num_nodes: usize, num_edges: usize) -> Self {
        let mut graph = Self::new();
        graph.randomize(num_nodes, num_edges);
        graph
    }

    pub fn get_random_node_id(&self) -> Uuid {
        let mut rng = rand::thread_rng();
        // Use the gen_range method to generate a random index
        let random_index = rng.gen_range(0..self.nodes.len());

        // Get the random element from the vector
        let random_element = self.nodes[random_index].id;
        random_element
    }
    pub fn get_neighbors(&self, node_id: Uuid) -> HashSet<Uuid> {
        let mut neighbors = HashSet::new();

        // Find the index of the node with the given ID in the node vector
        if let Some(&_) = self.node_lookup.get(&node_id) {
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
    pub fn get_node(&self, id: Uuid) -> Option<&GraphNode> {
        self.node_lookup
            .get(&id)
            .map(|index| self.nodes.get(*index))
            .unwrap_or(None)
    }
    pub fn get_coordinate_of_node(&self, id: Uuid) -> Option<&Coordinate> {
        self.get_node(id)
            .map(|node| Some(&node.meta.coordinate))
            .unwrap_or(None)
    }
    pub fn get_line_of_edge(&self, edge: &Edge) -> Option<Line> {
        self.get_coordinate_of_node(edge.incoming)
            .map(|in_coordinate| {
                self.get_coordinate_of_node(edge.outgoing)
                    .map(|out_coordinate| {
                        Some(Line {
                            from: *in_coordinate,
                            to: *out_coordinate,
                        })
                    })
                    .unwrap_or(None)
            })
            .unwrap_or(None)
    }
}
