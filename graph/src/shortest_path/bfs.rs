use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use uuid::Uuid;

use crate::Graph;

fn bfs(graph: &Graph, start_node_id: Uuid) -> Vec<Uuid> {
    // Create a queue for BFS and a HashSet to keep track of visited nodes
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Vector to store the BFS traversal result
    let mut result = Vec::new();

    // Check if the start node exists in the graph
    if let Some(&start_node_index) = graph.node_lookup.get(&start_node_id) {
        let start_node = &graph.nodes[start_node_index];
        queue.push_back(start_node);

        // Mark the start node as visited
        visited.insert(start_node_id);

        while !queue.is_empty() {
            // Dequeue a node from the front of the queue
            if let Some(node) = queue.pop_front() {
                // Add the node's ID to the result
                result.push(node.id);

                // Iterate over the outgoing edges of the current node
                for edge in &graph.edges {
                    if edge.incoming == node.id {
                        // Check if the target node of the edge is not visited
                        if !visited.contains(&edge.outgoing) {
                            // Mark the target node as visited and enqueue it
                            visited.insert(edge.outgoing);
                            if let Some(&next_node_index) = graph.node_lookup.get(&edge.outgoing) {
                                queue.push_back(&graph.nodes[next_node_index]);
                            }
                        }
                    }
                }
            }
        }
    }

    result
}

pub fn bfs_recursive(graph: &Graph, start_node_id: Uuid) -> HashMap<Uuid, Vec<Uuid>> {
    // Initialize a HashMap to store paths for each node.
    let mut paths: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

    // Initialize a queue for BFS and add the starting node to it.
    let mut queue: VecDeque<Uuid> = VecDeque::new();
    queue.push_back(start_node_id);

    // Initialize the path for the starting node as an empty vector.
    let mut current_path: Vec<Uuid> = Vec::new();
    current_path.push(start_node_id);
    paths.insert(start_node_id, current_path);

    // Call the recursive BFS function.
    bfs_recursive_helper(graph, &mut queue, &mut paths);

    paths
}

fn bfs_recursive_helper(
    graph: &Graph,
    queue: &mut VecDeque<Uuid>,
    paths: &mut HashMap<Uuid, Vec<Uuid>>,
) {
    if let Some(current_node_id) = queue.pop_front() {
        // Get the path to the current node.
        let current_path = paths.get(&current_node_id).unwrap().clone();

        // Iterate through outgoing edges of the current node.
        for edge in &graph.edges {
            if edge.incoming == current_node_id {
                let target_node_id = edge.outgoing;

                // If the target node is not in the paths HashMap, add it.
                if !paths.contains_key(&target_node_id) {
                    // Create a new path by extending the current path.
                    let mut new_path = current_path.clone();
                    new_path.push(target_node_id);

                    // Insert the new path into the paths HashMap.
                    paths.insert(target_node_id, new_path);

                    // Add the target node to the queue for further exploration.
                    queue.push_back(target_node_id);
                }
            }
        }

        // Continue the recursive BFS for the next node in the queue.
        bfs_recursive_helper(graph, queue, paths);
    }
}
