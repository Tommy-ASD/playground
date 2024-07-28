use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

use crate::graph::Graph;

pub fn dfs(graph: &Graph, start_node_id: Uuid) -> HashMap<Uuid, Vec<Uuid>> {
    let mut visited = HashMap::new();
    let path = vec![];
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
    path.push(current_node_id);
    visited.insert(current_node_id, path.clone());
    graph
        .get_neighbors(current_node_id)
        .iter()
        .for_each(|node_id| dfs_recursive_helper(graph, *node_id, visited, path.clone()));
}

pub fn bfs(graph: &Graph, start_node_id: Uuid, target_node_id: Uuid) -> Option<Vec<Uuid>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parent_map = HashMap::new();

    queue.push_back(start_node_id);
    visited.insert(start_node_id);

    while !queue.is_empty() {
        let current_node_id = queue.pop_front().unwrap();
        if current_node_id == target_node_id {
            return Some(reconstruct_path(&parent_map, start_node_id, target_node_id));
        }

        for neighbor_id in graph.get_neighbors(current_node_id) {
            if !visited.contains(&neighbor_id) {
                visited.insert(neighbor_id);
                queue.push_back(neighbor_id);
                parent_map.insert(neighbor_id, current_node_id);
            }
        }
    }

    None // No path found
}

fn reconstruct_path(
    parent_map: &HashMap<Uuid, Uuid>,
    start_node_id: Uuid,
    target_node_id: Uuid,
) -> Vec<Uuid> {
    let mut path = vec![];
    let mut current_node_id = target_node_id;
    while current_node_id != start_node_id {
        path.push(current_node_id);
        current_node_id = parent_map[&current_node_id];
    }
    path.push(start_node_id);
    path.reverse();
    path
}

pub fn one_to_all_shortest_path(
    graph: &Graph,
    source_id: Uuid,
) -> HashMap<Uuid, Option<Vec<Uuid>>> {
    graph
        .nodes
        .iter()
        .map(|node| (node.id, bfs(graph, source_id, node.id)))
        .collect::<HashMap<Uuid, Option<Vec<Uuid>>>>()
}

pub fn all_to_all_shortest_path(graph: &Graph) -> HashMap<Uuid, HashMap<Uuid, Option<Vec<Uuid>>>> {
    graph
        .nodes
        .iter()
        .map(|node| (node.id, one_to_all_shortest_path(graph, node.id)))
        .collect::<HashMap<Uuid, HashMap<Uuid, Option<Vec<Uuid>>>>>()
}
