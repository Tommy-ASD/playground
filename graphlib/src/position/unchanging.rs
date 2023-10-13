use std::collections::HashMap;

use crate::{meta::Coordinate, Graph};

use ordered_float::OrderedFloat;
use uuid::Uuid;

impl Graph {
    pub fn get_grid_positions(&self) -> HashMap<Uuid, Coordinate> {
        let mut node_positions = HashMap::new();
        // Calculate the number of rows and columns in the grid
        let num_nodes = self.nodes.len();
        let num_rows = (num_nodes as f64).sqrt().ceil() as usize;
        let num_columns = (num_nodes + num_rows - 1) / num_rows;

        // Calculate the step size for positioning nodes
        let step_x = OrderedFloat(2.0 / (num_columns as f64));
        let step_y = OrderedFloat(2.0 / (num_rows as f64));

        // Initialize node positions as a grid
        for (index, node) in self.nodes.iter().enumerate() {
            let row = index / num_columns;
            let col = index % num_columns;

            let x = OrderedFloat(-1.0) + (step_x * col as f64);
            let y = OrderedFloat(1.0) - (step_y * row as f64);

            node_positions.insert(node.id, Coordinate { x, y });
        }
        node_positions
    }
    pub fn get_circle_positions(&self) -> HashMap<Uuid, Coordinate> {
        let mut node_positions = HashMap::new();
        // Calculate the number of nodes
        let num_nodes = self.nodes.len();

        // Define the radius and center of the circle
        let radius = 1.0; // You can adjust this value for the desired circle size
        let center_x = 0.0;
        let center_y = 0.0;

        // Calculate the angle step between each node
        let angle_step = (2.0 * std::f64::consts::PI) / num_nodes as f64;

        // Initialize node positions in a circle
        for (index, node) in self.nodes.iter().enumerate() {
            let angle = angle_step * index as f64;
            let x = OrderedFloat(center_x + radius * angle.cos());
            let y = OrderedFloat(center_y + radius * angle.sin());

            node_positions.insert(node.id, Coordinate { x, y });
        }
        node_positions
    }
}
