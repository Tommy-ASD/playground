use std::collections::HashMap;

use crate::{meta::Coordinate, Graph};

use ordered_float::OrderedFloat;
use rand::Rng;
use uuid::Uuid;

impl Graph {
    fn calculate_repulsion(
        &self,
        node_id: Uuid,
        nodes: &mut HashMap<Uuid, Coordinate>,
        rep_strength: OrderedFloat<f64>,
    ) {
        let Coordinate { x: x1, y: y1 } = self.nodes[self.node_lookup[&node_id]].meta.coordinate;

        for (other_id, Coordinate { x: x2, y: y2 }) in nodes.iter_mut() {
            if *other_id != node_id {
                let dx = x1 - *x2;
                let dy = y1 - *y2;
                let distance_squared = dx * dx + dy * dy;
                if distance_squared > OrderedFloat(0.01) {
                    let force = rep_strength / distance_squared.sqrt();
                    *x2 += force * dx;
                    *y2 += force * dy;
                }
            }
        }
    }

    fn calculate_attraction(
        &self,
        source_id: Uuid,
        target_id: Uuid,
        nodes: &mut HashMap<Uuid, Coordinate>,
        spr_stiff: OrderedFloat<f64>,
    ) {
        let Coordinate { x: x1, y: y1 } = self.nodes[self.node_lookup[&source_id]].meta.coordinate;
        let Coordinate { x: x2, y: y2 } = self.nodes[self.node_lookup[&target_id]].meta.coordinate;

        let dx = x1 - x2;
        let dy = y1 - y2;
        let distance = (dx * dx + dy * dy).sqrt();
        let force = spr_stiff * (distance - 1.0);
        let force_x = force * dx / distance;
        let force_y = force * dy / distance;

        nodes.get_mut(&source_id).unwrap().x -= force_x;
        nodes.get_mut(&source_id).unwrap().y -= force_y;
        nodes.get_mut(&target_id).unwrap().x += force_x;
        nodes.get_mut(&target_id).unwrap().y += force_y;
    }

    pub fn full_fdl(&mut self, max_iterations: i32) {
        let mut rng = rand::thread_rng();
        let mut node_positions = self.get_grid_positions();
        self.apply_node_positions(&node_positions);

        // Initialize node positions randomly

        let mut old_pos: HashMap<Uuid, Coordinate> = HashMap::new();

        // Initialize node positions randomly
        for node in &self.nodes {
            old_pos.insert(
                node.id,
                Coordinate {
                    x: rng.gen_range(OrderedFloat(-1f64)..OrderedFloat(1f64)),
                    y: rng.gen_range(OrderedFloat(-1f64)..OrderedFloat(1f64)),
                },
            );
        }

        let mut iterations = 0;

        self.fdl(
            &mut iterations,
            max_iterations,
            &mut node_positions,
            &mut old_pos,
        );
    }
    fn fdl(
        &mut self,
        iterations: &mut i32,
        max_iterations: i32,
        node_positions: &mut HashMap<Uuid, Coordinate>,
        old_pos: &mut HashMap<Uuid, Coordinate>,
    ) {
        loop {
            // gloo::console::log!("10");
            self.calculate_next_force_iteration(
                node_positions,
                OrderedFloat(0.001),
                OrderedFloat(0.0005),
            );
            *iterations += 1;
            if *iterations > max_iterations {
                // gloo::console::log!("Ran ", *iterations, " iterations");
                break;
            }
            if *iterations % 100000 == 0 {
                // gloo::console::log!("At ", *iterations);
            }
            *old_pos = node_positions
                .iter()
                .map(|(node_id, Coordinate { x, y })| (*node_id, Coordinate { x: *x, y: *y }))
                .collect();
        }
        self.apply_node_positions(&node_positions);
    }
    pub fn calculate_next_force_iteration(
        &self,
        node_positions: &mut HashMap<Uuid, Coordinate>,
        rep_strength: OrderedFloat<f64>,
        spr_stiff: OrderedFloat<f64>,
    ) {
        // Calculate repulsion forces
        for node_id in self.node_lookup.keys() {
            self.calculate_repulsion(*node_id, node_positions, rep_strength);
        }

        // Calculate attraction forces for edges
        for edge in &self.edges {
            self.calculate_attraction(edge.incoming, edge.outgoing, node_positions, spr_stiff);
        }
    }
}
