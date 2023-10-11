use std::collections::HashMap;

use crate::Graph;

use ordered_float::OrderedFloat;
use rand::Rng;
use uuid::Uuid;

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

    pub fn force_directed_layout(&mut self, max_iterations: i32) {
        let mut rng = rand::thread_rng();
        let mut node_positions: HashMap<Uuid, (OrderedFloat<f64>, OrderedFloat<f64>)> =
            HashMap::new();

        // Initialize node positions randomly

        // Calculate the number of rows and columns in the grid
        let num_nodes = self.nodes.len();
        let num_rows = OrderedFloat(num_nodes as f64).sqrt().ceil() as usize;
        let num_columns = (num_nodes + num_rows - 1) / num_rows;

        // Calculate the step size for positioning nodes
        let step_x = OrderedFloat(2.0) / OrderedFloat(num_columns as f64);
        let step_y = OrderedFloat(2.0) / OrderedFloat(num_rows as f64);

        // Initialize node positions as a grid
        for (index, node) in self.nodes.iter().enumerate() {
            let row = index / num_columns;
            let col = index % num_columns;

            let x = OrderedFloat(-1.0) + (step_x * OrderedFloat(col as f64));
            let y = OrderedFloat(1.0) - (step_y * OrderedFloat(row as f64));

            node_positions.insert(node.id, (x, y));
        }
        // Update node positions based on forces
        for (node_id, (x, y)) in node_positions.iter() {
            let node_index = self.node_lookup[node_id];
            self.nodes[node_index].meta.position = (*x, *y);
            let id = self.nodes[node_index].id;
            println!("Node {id} got an initial position of {x}, {y}");
        }
        self.visualize_graph_with_path("initial.png");

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
        }

        let mut iterations = 0;
        let mut alpha = Self::INITIAL_ALPHA;

        while old_pos != node_positions {
            // alpha -= Self::ALPHA_DECREASE;
            iterations += 1;
            if iterations % 100000 == 0 {
                println!("At {iterations}");
            }
            if iterations > max_iterations {
                break;
            }
            old_pos = node_positions
                .iter()
                .map(|(node_id, (x, y))| (*node_id, (*x, *y)))
                .collect();
            // Calculate repulsion forces
            for node_id in self.node_lookup.keys() {
                // println!("Calculating repulsion for {node_id}");
                self.calculate_repulsion(*node_id, &mut node_positions, alpha);
            }

            // Calculate attraction forces for edges
            for edge in &self.edges {
                // println!(
                //     "Calculating attraction between {source} and {target}",
                //     source = edge.incoming,
                //     target = edge.outgoing
                // );
                self.calculate_attraction(edge.incoming, edge.outgoing, &mut node_positions, alpha);
            }
        }
        // Update node positions based on forces
        for (node_id, (x, y)) in node_positions.iter() {
            let node_index = self.node_lookup[node_id];
            self.nodes[node_index].meta.position = (*x, *y);
            let id = self.nodes[node_index].id;
            println!("Node {id} ended up at {x}, {y}");
        }
        println!("Ran {iterations} iterations of fdl");
    }
}
