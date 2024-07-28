use std::collections::HashMap;

use crate::{meta::Coordinate, Graph};

use uuid::Uuid;

pub mod force;
pub mod unchanging;

impl Graph {
    pub fn apply_node_positions(&mut self, node_positions: &HashMap<Uuid, Coordinate>) {
        for (node_id, Coordinate { x, y }) in node_positions.iter() {
            let node_index = self.node_lookup[node_id];
            self.nodes[node_index].meta.coordinate = Coordinate { x: *x, y: *y };
        }
    }
}
