use crate::{iter::*, util::*};

use graphlib::Graph;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use std::marker::PhantomData;

#[derive(Clone)]
pub enum LayoutType {
    ForceAtlas2,
    Force2,
    Fruchterman,
}

#[derive(Clone)]
pub enum DistanceThresholdMode {
    /// Use the average distance between nodes
    Average,
    /// Use the maximum distance between nodes
    Max,
    /// Use the minimum distance between nodes
    Min,
}

#[derive(Clone)]
pub struct Settings {
    pub name: LayoutType,
    /// Number of nodes computed by each thread
    ///
    /// Only used in repulsion computation. Set to `None` to turn off parallelization.
    /// This number should be big enough to minimize thread management,
    /// but small enough to maximize concurrency.
    ///
    /// Requires `T: Send + Sync`
    pub chunk_size: Option<usize>,
    /// Number of spatial dimensions
    pub dimensions: usize,
    /// Move hubs (high degree nodes) to the center
    pub dissuade_hubs: bool,
    /// Attraction coefficient
    pub ka: f32,
    /// Gravity coefficient
    pub kg: f32,
    /// Repulsion coefficient
    pub kr: f32,
    /// Logarithmic attraction
    pub lin_log: bool,
    /// Prevent node overlapping for a prettier graph (node_size, kr_prime).
    ///
    /// `node_size` is the radius around a node where the repulsion coefficient is `kr_prime`.
    /// `kr_prime` is arbitrarily set to `100.0` in Gephi implementation.
    pub prevent_overlapping: Option<(f32, f32)>,
    /// Speed factor
    pub speed: f32,
    /// Gravity does not decrease with distance, resulting in a more compact graph.
    pub strong_gravity: bool,

    /// Used in Force2 layout.
    pub link_distance: f32,
    /// The strength of edge force. Calculated according to the degree of nodes by default
    pub edge_strength: f32,
    /// The strength of node force. Positive value means repulsive force, negative value means attractive force (it is different from 'force')
    pub node_strength: f32,
    /// A parameter for repulsive force between nodes. Large the number, larger the repulsion.
    pub coulomb_dis_scale: f32,
    /// Coefficient for the repulsive force. Larger the number, larger the repulsive force.
    pub factor: f32,
    pub damping: f32,
    pub interval: f32,
    pub max_speed: f32,
    pub min_movement: f32,
    pub distance_threshold_mode: DistanceThresholdMode,
    pub max_distance: f32,

    /// Used in Fruchterman layout.
    pub center: Vec<f32>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            chunk_size: Some(256),
            dimensions: 2,
            dissuade_hubs: false,
            ka: 1.0,
            kg: 1.0,
            kr: 1.0,
            lin_log: false,
            prevent_overlapping: None,
            speed: 0.01,
            strong_gravity: false,
            name: LayoutType::ForceAtlas2,
            link_distance: 1.0,
            edge_strength: 1.0,
            node_strength: 1.0,
            coulomb_dis_scale: 1.0,
            factor: 1.0,
            damping: 1.0,
            interval: 1.0,
            center: vec![0.0; 2],
            max_speed: 1.0,
            min_movement: 0.0,
            distance_threshold_mode: DistanceThresholdMode::Average,
            max_distance: 100.0,
        }
    }
}

pub struct Layout {
    pub edges: Vec<Edge>,
    pub masses: Vec<f32>,
    /// List of the nodes' positions
    pub points: PointList,
    pub(crate) settings: Settings,
    pub speeds: PointList,
    pub old_speeds: PointList,
    pub weights: Option<Vec<f32>>,

    pub(crate) fn_attraction: fn(&mut Self),
    pub(crate) fn_gravity: fn(&mut Self),
    pub(crate) fn_repulsion: fn(&mut Self),
}

impl Layout {
    pub fn iter_nodes(&mut self) -> NodeIter {
        NodeIter {
            ind: 0,
            layout: SendPtr(self.into()),
            offset: 0,
            _phantom: PhantomData::default(),
        }
    }
}

impl Layout {
    pub fn iter_par_nodes(
        &mut self,
        chunk_size: usize,
    ) -> impl Iterator<Item = impl ParallelIterator<Item = NodeParIter>> {
        let ptr = SendPtr(self.into());
        let dimensions = self.settings.dimensions;
        let chunk_size_d = chunk_size * dimensions;
        let n = self.masses.len() * dimensions;
        (0..self.masses.len()).step_by(chunk_size).map(move |y0| {
            let y0_d = y0 * dimensions;
            (0..self.masses.len() - y0)
                .into_par_iter()
                .step_by(chunk_size)
                .map(move |x0| {
                    let x0_d = x0 * dimensions;
                    NodeParIter {
                        end: (x0_d + chunk_size_d).min(n),
                        ind: x0,
                        layout: ptr,
                        n2_start: x0_d + y0_d,
                        n2_start_ind: x0 + y0,
                        n2_end: (x0_d + y0_d + chunk_size_d).min(n),
                        offset: x0_d,
                        _phantom: PhantomData::default(),
                    }
                })
        })
    }

    pub fn from_graph(graph: &Graph) -> Self {
        let grid_positions = graph.get_grid_positions();
        let nodes = Nodes::Degree(graph.nodes.len());
        let mut index_to_coordinate: Vec<(usize, (f32, f32))> = grid_positions
            .iter()
            .map(|(k, v)| {
                let index = graph.node_lookup.get(k).unwrap();
                (*index, ((*v.x) as f32, (*v.y) as f32))
            })
            .collect();
        index_to_coordinate.sort_by(|a, b| a.0.cmp(&b.0));
        let positions: Vec<f32> = index_to_coordinate
            .iter()
            .flat_map(|(_, v)| vec![v.0, v.1])
            .collect();
        let edges = graph
            .edges
            .iter()
            .map(|edge| {
                (
                    graph.get_index_of_node_id(edge.incoming).unwrap(),
                    graph.get_index_of_node_id(edge.outgoing).unwrap(),
                )
            })
            .collect();
        let weights = None;
        // Create settings for the layout algorithm.
        let settings = Settings {
            name: LayoutType::Force2,
            // Set other layout parameters as needed
            chunk_size: Some(256),
            dimensions: 2,
            // Set other parameters
            distance_threshold_mode: DistanceThresholdMode::Average,
            ka: 0.01,
            kg: 0.01,
            kr: 0.01,
            dissuade_hubs: false,
            lin_log: false,
            prevent_overlapping: None,
            speed: 0.01,
            strong_gravity: false,
            link_distance: 1.0,
            edge_strength: 1.0,
            node_strength: 1.0,
            coulomb_dis_scale: 1.0,
            factor: 1.0,
            damping: 1.0,
            interval: 1.0,
            center: vec![0.0; 2],
            max_speed: 1.0,
            min_movement: 0.0,
            max_distance: 100.0,
        };
        let mut layout = Layout::from_position_graph(edges, nodes, positions, weights, settings);
        let num_iterations = 200; // Adjust as needed
        let progress_update = 1;
        for i in 0..num_iterations {
            if i % progress_update == 0 {
                println!("Running iteration {i}");
            }
            // Perform one iteration
            let done = layout.iteration(i);

            // You can check the 'done' flag to determine if the layout has converged.
            if done {
                println!("Layout converged after {} iterations", i);
                break;
            }
        }
        layout
    }

    pub fn apply_to_graph(&self, graph: &mut Graph) {
        for (node, position) in graph.nodes.iter_mut().zip(self.points.iter()) {
            // Update the node's position based on the layout results
            node.meta.coordinate.x = OrderedFloat(position[0] as f64);
            node.meta.coordinate.y = OrderedFloat(position[1] as f64);
        }
    }
}
