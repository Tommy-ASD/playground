use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use uuid::Uuid;

#[derive(Hash)]
struct GraphNode {
    outgoing: Vec<Uuid>,
    incoming: Vec<Uuid>,
    id: Uuid,
}

struct Graph {
    nodes: Vec<GraphNode>,
}

impl PartialEq for GraphNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn get_multi_mut<T>(v: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)> {
    if i == j {
        return None;
    }
    let (start, end) = if i < j { (i, j) } else { (j, i) };

    let (first, second) = v.split_at_mut(start + 1);
    Some((&mut first[start], &mut second[end - start - 1]))
}

impl Graph {
    fn add_outgoing_edge(&mut self, source_idx: Uuid, target_idx: Uuid) {
        let (source, target) = self.get_multi_mut(source_idx, target_idx).unwrap();
        source.outgoing.push(target.id);
        target.incoming.push(source.id);
    }
    fn add_incoming_edge(&mut self, source_idx: Uuid, target_idx: Uuid) {
        let (source, target) = self.get_multi_mut(source_idx, target_idx).unwrap();
        source.incoming.push(target.id);
        target.outgoing.push(source.id);
    }
    fn get_multi_mut(
        &mut self,
        source_id: Uuid,
        target_id: Uuid,
    ) -> Option<(&mut GraphNode, &mut GraphNode)> {
        let source_idx = self
            .nodes
            .iter()
            .enumerate()
            .find(|(_, node)| node.id == source_id)
            .map(|(index, _)| index);
        let target_idx = self
            .nodes
            .iter()
            .enumerate()
            .find(|(_, node)| node.id == target_id)
            .map(|(index, _)| index);
        let (source_idx, target_idx) = match (source_idx, target_idx) {
            (Some(source_idx), Some(target_idx)) => (source_idx, target_idx),
            _ => return None,
        };
        get_multi_mut(self.nodes.iter_mut().into_slice(), source_idx, target_idx)
    }
}

fn main() {
    // // Create some nodes
    // let node1 = Arc::new(Mutex::new(GraphNode::new()));
    // let node2 = Arc::new(Mutex::new(GraphNode::new()));
    // let node3 = Arc::new(Mutex::new(GraphNode::new()));

    // // Create edges
    // GraphNode::add_outgoing_edge(&node1, &node2);
    // GraphNode::add_outgoing_edge(&node2, &node3);
    // // GraphNode::add_outgoing_edge(&node3, &node1);

    // println!("Node 1: {node1:?}");
    // println!("Node 1 dfs: {dfs:?}", dfs = GraphNode::dfs(&node1));

    // // Perform operations on the graph
    // // ...
}
