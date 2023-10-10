use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use uuid::Uuid;

use crate::Graph;

#[derive(Debug, Eq, PartialEq)]
struct NodeDistance {
    node_id: Uuid,
    distance: OrderedFloat<f64>,
}

impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for BinaryHeap (min-heap)
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(graph: &Graph, source_id: Uuid, target_id: Uuid) -> Option<Vec<Uuid>> {
    println!("Running dijkstra for {source_id}, {target_id}");
    let mut distances: HashMap<Uuid, OrderedFloat<f64>> = HashMap::new();
    let mut previous: HashMap<Uuid, Uuid> = HashMap::new();
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new(); // Set to keep track of visited nodes

    // Initialize distances with infinity and the source node with 0.
    for node in &graph.nodes {
        distances.insert(node.id, OrderedFloat(std::f64::INFINITY));
    }
    distances.insert(source_id, OrderedFloat(0.0));

    queue.push(NodeDistance {
        node_id: source_id,
        distance: OrderedFloat(0.0),
    });

    while let Some(NodeDistance { node_id, distance }) = queue.pop() {
        if visited.contains(&node_id) {
            continue;
        }
        visited.insert(node_id);
        if node_id == target_id {
            println!("Found path from {source_id} to {target_id}");
            // Reconstruct the path from target to source
            let mut path = Vec::new();
            let mut current = target_id;
            while let Some(&prev) = previous.get(&current) {
                println!("{current} -> {prev}");
                path.push(current);
                current = prev;
            }
            path.push(source_id);
            path.reverse();
            return Some(path);
        }

        if distance > distances[&node_id] {
            continue;
        }

        for edge in &graph.edges {
            if edge.incoming == node_id {
                let neighbor_id = edge.outgoing;
                let neighbor_distance = distances[&node_id] + edge.meta.weight;

                if neighbor_distance < distances[&neighbor_id] {
                    distances.insert(neighbor_id, neighbor_distance);
                    previous.insert(neighbor_id, node_id);
                    let next = NodeDistance {
                        node_id: neighbor_id,
                        distance: neighbor_distance,
                    };
                    println!("Adding {next:?} to Dijkstra queue");
                    queue.push(next);
                }
            }
        }
    }

    // No path found
    None
}
