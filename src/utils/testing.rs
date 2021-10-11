use crate::alphabets::binary::{BinaryAlphabet, Phi, Psi};
use crate::markov::MarkovRandomField;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, UnGraph};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::{Distribution, Uniform};
use std::fs::File;
use std::io::Write;

fn generate_random_prufer_sequence<R: Rng>(mut rng: &mut R, n: usize) -> Vec<usize> {
    Uniform::from(0..n)
        .sample_iter(&mut rng)
        .take(n - 2)
        .collect()
}

fn prufer_to_tree(a: &[usize]) -> UnGraph<(), ()> {
    let n = a.len();
    let num_nodes = n + 2;
    let mut tree = UnGraph::default();
    for _ in 0..num_nodes {
        tree.add_node(());
    }

    let mut degree = vec![1; num_nodes];
    for &i in a {
        degree[i] += 1;
    }
    for &i in a {
        for j in 0..num_nodes {
            if degree[j] == 1 {
                tree.add_edge(NodeIndex::new(i), NodeIndex::new(j), ());
                degree[i] -= 1;
                degree[j] -= 1;
                break;
            }
        }
    }
    let mut u = 0;
    let mut v = 0;
    for i in 0..num_nodes {
        if degree[i] == 1 {
            if u == 0 {
                u = i;
            } else {
                v = i;
            }
        }
    }
    tree.add_edge(NodeIndex::new(u), NodeIndex::new(v), ());
    tree
}

pub fn random_tree<R: Rng>(num_nodes: usize, mut rng: &mut R) -> UnGraph<(), ()> {
    let prufer_sequence = generate_random_prufer_sequence(&mut rng, num_nodes);
    prufer_to_tree(prufer_sequence.as_slice())
}

pub fn random_binary_mrf<R: Rng>(
    num_nodes: usize,
    mut rng: &mut R,
) -> MarkovRandomField<BinaryAlphabet, Phi<BinaryAlphabet>, Psi> {
    let tree = random_tree(num_nodes, &mut rng);
    let unif = Uniform::new(0.0, 1.0);
    let alpha = unif.sample(&mut rng);
    let beta = unif.sample(&mut rng);
    let mut graph = UnGraph::default();
    for _ in 0..tree.node_count() {
        let node_potential = if unif.sample(&mut rng) < 0.2 {
            Phi::new_observed(beta, BinaryAlphabet::random(&mut rng))
        } else {
            Phi::new_unobserved(beta)
        };
        graph.add_node(node_potential);
    }

    for (from, to) in tree.edge_indices().filter_map(|e| tree.edge_endpoints(e)) {
        graph.add_edge(from, to, Psi::new(alpha));
    }

    MarkovRandomField::new(graph)
}
