use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use uuid::Uuid;

use crate::Graph;

pub fn dfs(graph: &Graph, start_node_id: Uuid) -> HashMap<Uuid, i32> {
    let mut visited = HashMap::new();
    dfs_recursive_helper(graph, start_node_id, &mut visited, 0);
    visited
}

fn dfs_recursive_helper(
    graph: &Graph,
    current_node_id: Uuid,
    visited: &mut HashMap<Uuid, i32>,
    depth: i32,
) {
    if let Some(v) = visited.get(&current_node_id) {
        if v < &depth {
            return;
        }
    }
    visited.insert(current_node_id, depth);
    graph
        .get_neighbors(current_node_id)
        .iter()
        .for_each(|node_id| dfs_recursive_helper(graph, *node_id, visited, depth + 1));
}
