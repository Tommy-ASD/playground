use ndarray::prelude::{Array1, Array2};
use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::Graph;

/// Performs Dijkstra's algorithm on a directed graph to compute shortest distances from a single
/// source node to all other nodes in the graph, storing the results in a distance matrix.
///
/// Dijkstra's algorithm is used to find the shortest path from a specified source node to all
/// other nodes in a weighted directed graph. It computes the shortest distances from the source
/// node to all other reachable nodes and stores these distances in a matrix.
///
/// # Arguments
///
/// * `graph` - A reference to the directed graph to perform the algorithm on.
/// * `indices` - A reference to a `HashMap` that maps node identifiers to their corresponding
///   indices in the `distance_matrix`. The node identifiers are of type `u32`.
/// * `s` - The starting node from which to calculate the shortest distances.
/// * `distance_matrix` - A mutable reference to a 2D array (`Array2<f32>`) to store the computed
///   shortest distances. The dimensions of the array should match the number of nodes in the
///   graph.
/// * `k` - An index indicating the column in the `distance_matrix` where the computed distances
///   should be stored.
///
/// # Example
///
/// ```rust
/// use kamada_kawai::{Graph, dijkstra_with_distance_matrix};
/// use ndarray::prelude::{Array2};
/// use std::collections::HashMap;
///
/// // Create a graph with nodes and edges.
/// let mut graph = Graph::new();
/// graph.add_node(0);
/// graph.add_node(1);
/// graph.add_edge(0, 1, 5.0);
///
/// // Create a mapping of node identifiers to matrix indices.
/// let mut indices = HashMap::new();
/// indices.insert(0, 0);
/// indices.insert(1, 1);
///
/// // Initialize a distance matrix with initial values.
/// let mut distance_matrix = Array2::from_elem((2, 2), f32::INFINITY);
///
/// // Calculate shortest distances using Dijkstra's algorithm.
/// dijkstra_with_distance_matrix(&graph, &indices, 0, &mut distance_matrix, 0);
///
/// println!("{distance_matrix:?}");
/// ```
pub fn dijkstra_with_distance_matrix(
    graph: &Graph,
    indices: &HashMap<u32, usize>,
    s: u32,
    distance_matrix: &mut Array2<f32>,
    k: usize,
) {
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(OrderedFloat(0.)), s));
    distance_matrix[[indices[&s], k]] = 0.;

    while let Some((Reverse(OrderedFloat(d)), u)) = queue.pop() {
        for &(v, w, weight) in &graph.edges {
            if u == v {
                let e = d + weight;
                if e < distance_matrix[[indices[&w], k]] {
                    queue.push((Reverse(OrderedFloat(e)), w));
                    distance_matrix[[indices[&w], k]] = e;
                }
            }
        }
    }
}

/// Calculates shortest distances from multiple source nodes to all other nodes in a directed graph
/// and stores the results in a distance matrix.
///
/// This function extends Dijkstra's algorithm to compute the shortest distances from a set of
/// source nodes to all other nodes in a weighted directed graph. The results are stored in a
/// distance matrix where each column corresponds to one of the source nodes, and each row
/// represents a destination node.
///
/// # Arguments
///
/// * `graph` - A reference to the directed graph to perform the algorithm on.
/// * `sources` - A slice containing the identifiers of source nodes from which to calculate
///   shortest distances. The identifiers are of type `u32`.
///
/// # Returns
///
/// A 2D array (`Array2<f32>`) representing the computed shortest distances. The dimensions of
/// the array match the number of nodes in the graph (rows) and the number of source nodes (columns).
///
/// # Example
///
/// ```rust
/// use my_graph_library::{Graph, multi_source_dijkstra};
/// use ndarray::prelude::{Array2};
///
/// // Create a graph with nodes and edges.
/// let mut graph = Graph::new();
/// graph.add_node(0);
/// graph.add_node(1);
/// graph.add_node(2);
/// graph.add_edge(0, 1, 5.0);
/// graph.add_edge(0, 2, 8.0);
///
/// // Calculate shortest distances from multiple sources.
/// let sources = vec![0, 1];
/// let distance_matrix = multi_source_dijkstra(&graph, &sources);
/// ```
pub fn multi_source_dijkstra(
    graph: &Graph,   // Pass a reference to your custom Graph struct.
    sources: &[u32], // Updated to use u32 for node IDs.
) -> Array2<f32> {
    let indices = graph
        .nodes
        .iter()
        .enumerate()
        .map(|(i, u)| (*u, i))
        .collect::<HashMap<u32, usize>>(); // Use a reference to u32.
    let n = indices.len();
    let k = sources.len();
    let mut distance_matrix = Array2::from_elem((n, k), f32::INFINITY);

    for c in 0..k {
        dijkstra_with_distance_matrix(graph, &indices, sources[c], &mut distance_matrix, c);
    }
    distance_matrix
}

/// Calculates shortest distances from all nodes in a directed graph to all other nodes and
/// stores the results in a distance matrix.
///
/// This function utilizes the `multi_source_dijkstra` function to compute the shortest distances
/// from every node in the graph to all other nodes. The results are stored in a distance matrix
/// where each column represents one of the source nodes, and each row represents a destination node.
///
/// # Arguments
///
/// * `graph` - A reference to the directed graph to perform the algorithm on.
///
/// # Returns
///
/// A 2D array (`Array2<f32>`) representing the computed shortest distances. The dimensions of
/// the array match the number of nodes in the graph (rows) and the total number of nodes in the
/// graph (columns).
///
/// # Example
///
/// ```rust
/// use my_graph_library::{Graph, all_sources_dijkstra};
/// use ndarray::prelude::{Array2};
///
/// // Create a graph with nodes and edges.
/// let mut graph = Graph::new();
/// graph.add_node(0);
/// graph.add_node(1);
/// graph.add_node(2);
/// graph.add_edge(0, 1, 5.0);
/// graph.add_edge(1, 2, 3.0);
///
/// // Calculate shortest distances from all nodes.
/// let distance_matrix = all_sources_dijkstra(&graph);
/// ``
pub fn all_sources_dijkstra(
    graph: &Graph, // Pass a reference to your custom Graph struct.
) -> Array2<f32> {
    let sources = &graph.nodes; // Use the nodes directly from your Graph struct.
    multi_source_dijkstra(graph, sources)
}

/// Calculates the shortest distances from a single source node to all other nodes in a directed graph
/// and stores the results in an array.
///
/// This function utilizes the `multi_source_dijkstra` function to compute the shortest distances from
/// a specified source node to all other nodes in the graph. The results are stored in a 1D array where
/// each element represents the shortest distance from the source node to the corresponding destination
/// node.
///
/// # Arguments
///
/// * `graph` - A reference to the directed graph to perform the algorithm on.
/// * `s` - The identifier of the source node from which to calculate the shortest distances. The
///   identifier is of type `u32`.
///
/// # Returns
///
/// A 1D array (`Array1<f32>`) representing the computed shortest distances from the source node
/// to all other nodes in the graph.
///
/// # Example
///
/// ```rust
/// use my_graph_library::{Graph, dijkstra};
/// use ndarray::prelude::{Array1};
///
/// // Create a graph with nodes and edges.
/// let mut graph = Graph::new();
/// graph.add_node(0);
/// graph.add_node(1);
/// graph.add_node(2);
/// graph.add_edge(0, 1, 5.0);
/// graph.add_edge(1, 2, 3.0);
///
/// // Calculate shortest distances from a single source.
/// let source_node = 0;
/// let distances = dijkstra(&graph, source_node);
/// ```
pub fn dijkstra(
    graph: &Graph, // Pass a reference to your custom Graph struct.
    s: u32,        // Updated to use u32 for node IDs.
) -> Array1<f32> {
    let distance_matrix = multi_source_dijkstra(graph, &[s]);
    let n = distance_matrix.shape()[0];
    distance_matrix.into_shape(n).unwrap()
}
