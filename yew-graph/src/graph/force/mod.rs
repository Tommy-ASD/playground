use std::collections::HashMap;

use crate::{canvas::inner::CanvasBackend, get_canvas, get_state, graph::Graph, State};

use ordered_float::OrderedFloat;
use plotters::prelude::IntoDrawingArea;
use rand::Rng;
use uuid::Uuid;
use web_sys::HtmlCanvasElement;

impl Graph {
    const REPULSION_STRENGTH: OrderedFloat<f64> = OrderedFloat(0.001);
    const SPRING_STIFFNESS: OrderedFloat<f64> = OrderedFloat(0.0005);
    const INITIAL_ALPHA: OrderedFloat<f64> = OrderedFloat(0.95);
    const ALPHA_DECREASE: OrderedFloat<f64> = OrderedFloat(0.005);

    fn calculate_repulsion(
        &self,
        node_id: Uuid,
        nodes: &mut HashMap<Uuid, (OrderedFloat<f64>, OrderedFloat<f64>)>,
        alpha: OrderedFloat<f64>,
    ) {
        let (x1, y1) = self.nodes[self.node_lookup[&node_id]].meta.position;

        for (other_id, (x2, y2)) in nodes.iter_mut() {
            if *other_id != node_id {
                let dx = x1 - *x2;
                let dy = y1 - *y2;
                let distance_squared = dx * dx + dy * dy;
                if distance_squared > OrderedFloat(0.01) {
                    let force = Graph::REPULSION_STRENGTH / distance_squared.sqrt() * alpha;
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
        nodes: &mut HashMap<Uuid, (OrderedFloat<f64>, OrderedFloat<f64>)>,
        alpha: OrderedFloat<f64>,
    ) {
        let (x1, y1) = self.nodes[self.node_lookup[&source_id]].meta.position;
        let (x2, y2) = self.nodes[self.node_lookup[&target_id]].meta.position;

        let dx = x1 - x2;
        let dy = y1 - y2;
        let distance = (dx * dx + dy * dy).sqrt();
        let force = Graph::SPRING_STIFFNESS * (distance - 1.0) * alpha;
        let force_x = force * dx / distance;
        let force_y = force * dy / distance;

        nodes.get_mut(&source_id).unwrap().0 -= force_x;
        nodes.get_mut(&source_id).unwrap().1 -= force_y;
        nodes.get_mut(&target_id).unwrap().0 += force_x;
        nodes.get_mut(&target_id).unwrap().1 += force_y;
    }

    fn initialize_positions(
        &mut self,
        node_positions: &mut HashMap<Uuid, (OrderedFloat<f64>, OrderedFloat<f64>)>,
    ) {
        // Calculate the number of rows and columns in the grid
        let num_nodes = self.nodes.len();
        let num_rows = (num_nodes as f64).sqrt().ceil() as usize;
        let num_columns = (num_nodes + num_rows - 1) / num_rows;

        // Calculate the step size for positioning nodes
        let step_x = OrderedFloat(2.0 / (num_columns as f64));
        let step_y = OrderedFloat(2.0 / (num_rows as f64));
        gloo::console::log!("3");

        // Initialize node positions as a grid
        for (index, node) in self.nodes.iter().enumerate() {
            gloo::console::log!("4");
            let row = index / num_columns;
            let col = index % num_columns;

            let x = OrderedFloat(-1.0) + (step_x * col as f64);
            let y = OrderedFloat(1.0) - (step_y * row as f64);

            node_positions.insert(node.id, (x, y));
        }
        // Update node positions based on forces
        for (node_id, (x, y)) in node_positions.iter() {
            gloo::console::log!("5");
            let node_index = self.node_lookup[node_id];
            self.nodes[node_index].meta.position = (*x, *y);
            let id = self.nodes[node_index].id;
            gloo::console::log!(
                "Node",
                id.to_string(),
                "got an initial position of ",
                x.to_string(),
                ", ",
                y.to_string()
            );
        }
    }

    pub fn full_fdl(&mut self, max_iterations: i32) {
        gloo::console::log!("1");
        let mut rng = rand::thread_rng();
        let mut node_positions: HashMap<Uuid, (OrderedFloat<f64>, OrderedFloat<f64>)> =
            HashMap::new();
        self.initialize_positions(&mut node_positions);
        gloo::console::log!("2");

        // Initialize node positions randomly

        let mut old_pos: HashMap<Uuid, (OrderedFloat<f64>, OrderedFloat<f64>)> = HashMap::new();

        // Initialize node positions randomly
        for node in &self.nodes {
            old_pos.insert(
                node.id,
                (
                    rng.gen_range(OrderedFloat(-1f64)..OrderedFloat(1f64)),
                    rng.gen_range(OrderedFloat(-1f64)..OrderedFloat(1f64)),
                ),
            );
            gloo::console::log!("6");
        }

        let mut iterations = 0;
        let mut alpha = Self::INITIAL_ALPHA;

        self.fdl(
            &mut iterations,
            max_iterations,
            &mut node_positions,
            &mut old_pos,
            &mut alpha,
        );

        gloo::console::log!("Ran", iterations, "iterations of fdl");
    }
    fn fdl(
        &mut self,
        iterations: &mut i32,
        max_iterations: i32,
        node_positions: &mut HashMap<Uuid, (OrderedFloat<f64>, OrderedFloat<f64>)>,
        old_pos: &mut HashMap<Uuid, (OrderedFloat<f64>, OrderedFloat<f64>)>,
        alpha: &mut OrderedFloat<f64>,
    ) {
        let root = get_canvas().unwrap();
        loop {
            gloo::console::log!("10");
            if let None = self.next_force_iteration(
                iterations,
                max_iterations,
                node_positions,
                old_pos,
                alpha,
            ) {
                gloo::console::log!("15");
                break;
            }
            gloo::console::log!("11");
            // Update node positions based on forces
            gloo::console::log!("13");
            self.draw_on_backend(&root).unwrap();
            gloo::console::log!("14");
        }
        for (node_id, (x, y)) in node_positions.iter() {
            gloo::console::log!("12");
            let node_index = self.node_lookup[node_id];
            self.nodes[node_index].meta.position = (*x, *y);
            let id = self.nodes[node_index].id;
            gloo::console::log!(
                "Node",
                id.to_string(),
                "ended up at",
                x.to_string(),
                ", ",
                y.to_string()
            );
        }
    }
    fn next_force_iteration(
        &self,
        iterations: &mut i32,
        max_iterations: i32,
        node_positions: &mut HashMap<Uuid, (OrderedFloat<f64>, OrderedFloat<f64>)>,
        old_pos: &mut HashMap<Uuid, (OrderedFloat<f64>, OrderedFloat<f64>)>,
        alpha: &mut OrderedFloat<f64>,
    ) -> Option<()> {
        gloo::console::log!("1");
        // alpha -= Self::ALPHA_DECREASE;
        *iterations += 1;
        if *iterations % 100000 == 0 {
            gloo::console::log!("At ", *iterations);
        }
        if *iterations > max_iterations {
            gloo::console::log!("6");
            return None;
        }
        gloo::console::log!("2");
        *old_pos = node_positions
            .iter()
            .map(|(node_id, (x, y))| (*node_id, (*x, *y)))
            .collect();
        // Calculate repulsion forces
        for node_id in self.node_lookup.keys() {
            gloo::console::log!("3");
            // println!("Calculating repulsion for {node_id}");
            self.calculate_repulsion(*node_id, node_positions, *alpha);
        }

        // Calculate attraction forces for edges
        for edge in &self.edges {
            gloo::console::log!("4");
            // println!(
            //     "Calculating attraction between {source} and {target}",
            //     source = edge.incoming,
            //     target = edge.outgoing
            // );
            self.calculate_attraction(edge.incoming, edge.outgoing, node_positions, *alpha);
        }
        gloo::console::log!("5");
        return Some(());
    }
}
