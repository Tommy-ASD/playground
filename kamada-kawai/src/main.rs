use drawing::{Drawing, DrawingIndex};
use ndarray::Array2;
use petgraph::visit::{IntoEdges, IntoNodeIdentifiers, NodeCount};
use shortest_path::{all_sources_dijkstra, warshall_floyd};

mod drawing;
mod shortest_path;

fn norm(x: f32, y: f32) -> f32 {
    (x * x + y * y).sqrt().max(1.)
}

pub struct KamadaKawai {
    k_matrix: Array2<f32>,
    l_matrix: Array2<f32>,
    pub epsilon: f32,
}

impl KamadaKawai {
    pub fn new<G, F>(graph: G, edge_length: F) -> KamadaKawai
    where
        G: IntoEdges + IntoNodeIdentifiers + NodeCount,
        G::NodeId: DrawingIndex + Ord,
        F: FnMut(G::EdgeRef) -> f32,
    {
        let length_matrix = all_sources_dijkstra(graph, edge_length);
        KamadaKawai::new_with_distance_matrix(&length_matrix)
    }

    pub fn new_with_distance_matrix(length_matrix: &Array2<f32>) -> KamadaKawai {
        let epsilon = 1e-1;
        let n = length_matrix.nrows();

        let mut stiffness_matrix = Array2::zeros((n, n));
        for i in 0..n {
            for j in 0..n {
                stiffness_matrix[[i, j]] = 1. / (length_matrix[[i, j]] * length_matrix[[i, j]]);
            }
        }
        KamadaKawai {
            k_matrix: stiffness_matrix,
            l_matrix: length_matrix.clone(),
            epsilon,
        }
    }

    pub fn select_node<N>(&self, drawing: &Drawing<N, f32>) -> Option<usize>
    where
        N: DrawingIndex,
    {
        let n = drawing.len();
        let KamadaKawai {
            k_matrix,
            l_matrix,
            epsilon,
            ..
        } = self;
        let mut max_delta_squared = 0.;
        let mut target_node = 0;
        for m in 0..n {
            let xm = drawing.coordinates[[m, 0]];
            let ym = drawing.coordinates[[m, 1]];
            let mut dedx = 0.;
            let mut dedy = 0.;
            for i in 0..n {
                if i != m {
                    let xi = drawing.coordinates[[i, 0]];
                    let yi = drawing.coordinates[[i, 1]];
                    let dx = xm - xi;
                    let dy = ym - yi;
                    let d = norm(dx, dy);
                    dedx += k_matrix[[m, i]] * (1. - l_matrix[[m, i]] / d) * dx;
                    dedy += k_matrix[[m, i]] * (1. - l_matrix[[m, i]] / d) * dy;
                }
            }
            let delta_squared = dedx * dedx + dedy * dedy;
            if delta_squared > max_delta_squared {
                max_delta_squared = delta_squared;
                target_node = m;
            }
        }

        if max_delta_squared < epsilon * epsilon {
            None
        } else {
            Some(target_node)
        }
    }

    pub fn apply_to_node<N>(&self, m: usize, drawing: &mut Drawing<N, f32>)
    where
        N: DrawingIndex,
    {
        let n = drawing.len();
        let KamadaKawai {
            k_matrix, l_matrix, ..
        } = self;
        let xm = drawing.coordinates[[m, 0]];
        let ym = drawing.coordinates[[m, 1]];
        let mut hxx = 0.;
        let mut hyy = 0.;
        let mut hxy = 0.;
        let mut dedx = 0.;
        let mut dedy = 0.;
        for i in 0..n {
            if i != m {
                let xi = drawing.coordinates[[i, 0]];
                let yi = drawing.coordinates[[i, 1]];
                let dx = xm - xi;
                let dy = ym - yi;
                let d = norm(dx, dy);
                let d_cubed = d * d * d;
                hxx += k_matrix[[m, i]] * (1. - l_matrix[[m, i]] * dy * dy / d_cubed);
                hyy += k_matrix[[m, i]] * (1. - l_matrix[[m, i]] * dx * dx / d_cubed);
                hxy += k_matrix[[m, i]] * l_matrix[[m, i]] * dx * dy / d_cubed;
                dedx += k_matrix[[m, i]] * (1. - l_matrix[[m, i]] / d) * dx;
                dedy += k_matrix[[m, i]] * (1. - l_matrix[[m, i]] / d) * dy;
            }
        }
        let determinant = hxx * hyy - hxy * hxy;
        let delta_x = (hyy * dedx - hxy * dedy) / determinant;
        let delta_y = (hxx * dedy - hxy * dedx) / determinant;
        drawing.coordinates[[m, 0]] -= delta_x;
        drawing.coordinates[[m, 1]] -= delta_y;
    }

    pub fn run<N>(&self, drawing: &mut Drawing<N, f32>)
    where
        N: DrawingIndex,
    {
        while let Some(m) = self.select_node(drawing) {
            self.apply_to_node(m, drawing);
        }
    }
}

#[test]
fn test_kamada_kawai() {
    use petgraph::Graph;

    let n = 10;
    let mut graph = Graph::new_undirected();
    let nodes = (0..n).map(|_| graph.add_node(())).collect::<Vec<_>>();
    for i in 0..n {
        for j in 0..i {
            graph.add_edge(nodes[j], nodes[i], ());
        }
    }

    let mut coordinates = Drawing::initial_placement(&graph);

    for &node in &nodes {
        println!("{:?}", coordinates.position(node));
    }

    let kamada_kawai = KamadaKawai::new(&graph, &mut |_| 1.);
    kamada_kawai.run(&mut coordinates);

    for &node in &nodes {
        println!("{:?}", coordinates.position(node));
    }
}

fn main() {
    use petgraph::Graph;

    let n = 10;
    let mut graph = Graph::new_undirected();
    let nodes = (0..n).map(|_| graph.add_node(())).collect::<Vec<_>>();
    for i in 0..n {
        for j in 0..i {
            graph.add_edge(nodes[j], nodes[i], ());
        }
    }

    let mut coordinates = Drawing::initial_placement(&graph);

    for &node in &nodes {
        println!("{:?}", coordinates.position(node));
    }

    let kamada_kawai = KamadaKawai::new(&graph, &mut |_| 1.);
    kamada_kawai.run(&mut coordinates);

    for &node in &nodes {
        println!("{:?}", coordinates.position(node));
    }
}
