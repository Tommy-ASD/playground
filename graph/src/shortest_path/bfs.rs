use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use uuid::Uuid;

use crate::Graph;

// Define a TreeNode structure to represent the tree structure of paths.
#[derive(Clone)]
struct TreeNode {
    node_id: Uuid,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(node_id: Uuid) -> Self {
        TreeNode {
            node_id,
            children: Vec::new(),
        }
    }

    // Recursive function to add a node to the tree.
    fn add_child(&mut self, child_node: TreeNode) {
        self.children.push(child_node);
    }

    // Convert the tree to a flattened HashMap of paths.
    fn to_path_map(&self) -> HashMap<Uuid, Vec<Uuid>> {
        let mut path_map: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        self.to_path_vector(Vec::new(), &mut path_map);
        path_map
    }

    // Recursively traverse the tree to construct paths and populate the HashMap.
    fn to_path_vector(&self, mut current_path: Vec<Uuid>, path_map: &mut HashMap<Uuid, Vec<Uuid>>) {
        current_path.push(self.node_id);
        path_map.insert(self.node_id, current_path.clone());

        for child in &self.children {
            child.to_path_vector(current_path.clone(), path_map);
        }
    }
}

pub fn bfs_recursive(graph: &Graph, start_node_id: Uuid) -> HashMap<Uuid, Vec<Uuid>> {
    // Initialize a HashMap to store paths for each node.
    let mut paths: HashMap<Uuid, TreeNode> = HashMap::new();

    // Initialize a queue for BFS and add the starting node to it.
    let mut queue: VecDeque<Uuid> = VecDeque::new();
    queue.push_back(start_node_id);

    // Initialize the path for the starting node as a single-node TreeNode.
    let start_node_tree = TreeNode::new(start_node_id);
    paths.insert(start_node_id, start_node_tree);

    // Call the recursive BFS function.
    bfs_recursive_helper(graph, &mut queue, &mut paths);

    // Convert the tree structure to a HashMap of paths.
    paths
        .values()
        .map(|tree| tree.to_path_map())
        .fold(HashMap::new(), |mut acc, map| {
            acc.extend(map);
            acc
        })
}

fn bfs_recursive_helper(
    graph: &Graph,
    queue: &mut VecDeque<Uuid>,
    paths: &mut HashMap<Uuid, TreeNode>,
) {
    while let Some(current_node_id) = queue.pop_front() {
        let current_tree_node = paths.get(&current_node_id).unwrap().clone();

        // Iterate through outgoing edges of the current node.
        for edge in &graph.edges {
            if edge.incoming == current_node_id {
                let target_node_id = edge.outgoing;

                // If the target node is not in the paths HashMap, add it.
                if !paths.contains_key(&target_node_id) {
                    // Create a new path by extending the current path.
                    let mut new_tree_node = current_tree_node.clone();
                    let child_tree_node = TreeNode::new(target_node_id);
                    new_tree_node.add_child(child_tree_node);

                    // Insert the new path into the paths HashMap.
                    paths.insert(target_node_id, new_tree_node.clone());

                    // Add the target node to the queue for further exploration.
                    queue.push_back(target_node_id);
                }
            }
        }
    }
}
