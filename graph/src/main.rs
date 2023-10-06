use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

static mut GRAPH: Vec<ArcNode> = vec![];

type ArcNode = Arc<Mutex<GraphNode>>;

#[derive(Debug)]
struct GraphNode {
    id: i32,
    incoming: Vec<ArcNode>,
    outgoing: Vec<ArcNode>,
}

impl PartialEq for GraphNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl GraphNode {
    fn new() -> Self {
        unsafe {
            let id;
            id = GRAPH.len();

            let node = GraphNode {
                id: id.try_into().unwrap(),
                incoming: Vec::new(),
                outgoing: Vec::new(),
            };
            node
        }
    }

    fn add_outgoing_edge(source: &ArcNode, target: &ArcNode) {
        source.lock().unwrap().outgoing.push(Arc::clone(target));
        target.lock().unwrap().incoming.push(Arc::clone(source));
    }

    fn add_incoming_edge(target: &ArcNode, source: &ArcNode) {
        source.lock().unwrap().outgoing.push(Arc::clone(target));
        target.lock().unwrap().incoming.push(Arc::clone(source));
    }

    fn dfs(source: &ArcNode) -> Vec<i32> {
        println!("Initializing dfs");
        let locked = source.lock().unwrap();
        println!("Got a lock on source");
        drop(locked);
        let mut visited = vec![];
        println!("Initializing dfs_inner");
        GraphNode::dfs_inner(source, &mut visited);
        println!("Finished dfs_inner");
        visited
    }

    fn dfs_inner(source: &ArcNode, visited: &mut Vec<i32>) {
        println!("dfs_inner: Trying to get a lock");
        let lock = source.lock().unwrap();
        println!("Calculating {lock:?}");
        if visited.contains(&lock.id) {
            return;
        }
        lock.incoming.iter().for_each(|node| {
            println!("Calculating incoming node {node:?}");
            GraphNode::dfs_inner(node, visited);
            visited.push(node.lock().unwrap().id);
        });
        lock.outgoing.iter().for_each(|node| {
            println!("Calculating outgoing node {node:?}");
            GraphNode::dfs_inner(node, visited);
            visited.push(node.lock().unwrap().id);
        });
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
    // GraphNode::add_outgoing_edge(&node3, &node1);

    println!("Node 1: {node1:?}");
    println!("Node 1 dfs: {dfs:?}", dfs = GraphNode::dfs(&node1));

    // Perform operations on the graph
    // ...
}
