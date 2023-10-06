use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

static mut CURRENT_GRAPH_NODE_ID: i32 = 0;

type ArcNode = Arc<Mutex<GraphNode>>;

#[derive(Debug)]
struct GraphNode {
    id: i32,
    incoming: Vec<ArcNode>,
    outgoing: Vec<ArcNode>,
}

impl GraphNode {
    fn new() -> Self {
        let id;
        unsafe {
            id = CURRENT_GRAPH_NODE_ID;
            CURRENT_GRAPH_NODE_ID += 1;
        }
        GraphNode {
            id,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }

    fn add_outgoing_edge(source: &Arc<Mutex<Self>>, target: &ArcNode) {
        source.lock().unwrap().outgoing.push(Arc::clone(target));
        target.lock().unwrap().incoming.push(Arc::clone(source));
    }

    fn add_incoming_edge(target: &Arc<Mutex<Self>>, source: &ArcNode) {
        source.lock().unwrap().outgoing.push(Arc::clone(target));
        target.lock().unwrap().incoming.push(Arc::clone(source));
    }
}

fn main() {
    // Create some nodes
    let node1 = Arc::new(Mutex::new(GraphNode::new()));
    let node2 = Arc::new(Mutex::new(GraphNode::new()));
    let node3 = Arc::new(Mutex::new(GraphNode::new()));

    // Create edges
    GraphNode::add_outgoing_edge(&node1, &node2);
    GraphNode::add_outgoing_edge(&node2, &node3);
    GraphNode::add_outgoing_edge(&node3, &node1);

    println!("Node 1: {node1:?}");

    // Perform operations on the graph
    // ...
}
