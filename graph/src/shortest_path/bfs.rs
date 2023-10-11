use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use uuid::Uuid;

use crate::Graph;

pub fn dfs(graph: &Graph, start_node_id: Uuid) -> HashMap<Uuid, Vec<Uuid>> {
    let mut visited = HashMap::new();
    let mut path = vec![start_node_id];
    dfs_recursive_helper(graph, start_node_id, &mut visited, path);
    visited
}

fn dfs_recursive_helper(
    graph: &Graph,
    current_node_id: Uuid,
    visited: &mut HashMap<Uuid, Vec<Uuid>>,
    mut path: Vec<Uuid>,
) {
    if let Some(v) = visited.get(&current_node_id) {
        if v.len() < path.len() {
            return;
        }
    }
    visited.insert(current_node_id, path.clone());
    path.push(current_node_id);
    graph
        .get_neighbors(current_node_id)
        .iter()
        .for_each(|node_id| dfs_recursive_helper(graph, *node_id, visited, path.clone()));
}
