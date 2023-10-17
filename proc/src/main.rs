use macros::{Getters, GettersMut, Setters};

fn main() {
    println!("Hello, world!");
}

#[derive(Setters)]
pub enum DistanceThresholdMode {
    /// Use the average distance between nodes
    Average,
    /// Use the maximum distance between nodes
    Max,
    /// Use the minimum distance between nodes
    Min,
}

#[derive(Setters)]
pub enum LayoutType {
    ForceAtlas2,
    Force2,
    Fruchterman,
}

#[derive(Setters, Getters, GettersMut)]
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
    /// Repulsion coefficients
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
