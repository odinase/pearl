use itertools::Itertools;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, UnGraph, NodeIndex};
use petgraph::Direction;

fn moralize<N>(mut graph: DiGraph<N, ()>) -> UnGraph<N, ()> {
    for i in graph.node_indices() {
        let num_parents = graph.neighbors_directed(i, Direction::Incoming).count();
        if num_parents > 1 {
            // The node has parents that needs to be moralized
            let new_edges: Vec<_> = graph
                .neighbors_directed(i, Direction::Incoming) // All parents
                .combinations(2) // Create pairwise parents that need moralizing
                .filter(|v| {
                    !graph.contains_edge(v[0], v[1]) && !graph.contains_edge(v[1], v[0]) // Filter out already connected parents
                })
                .map(&|v: Vec<NodeIndex<_>>| (v[0], v[1])) // Unpack internal vectors into tuples for convenience
                .collect();
            let new_parents_iter = new_edges.iter().copied();
            for (from, to) in new_parents_iter {
                graph.update_edge(from, to, ());
            }
        }
    }
    graph.into_edge_type()
}

fn main() {
    let digraph: DiGraph<i32, ()> =
        DiGraph::from_edges(&[(0, 3), (2, 3), (1, 4), (3, 4), (1, 5), (4, 5), (0, 5)]);

    println!(
        "Directed graph:\n{:?}",
        Dot::with_config(&digraph, &[Config::EdgeNoLabel, Config::NodeIndexLabel])
    );

    let moralized_graph = moralize(digraph);
    println!(
        "Moralized graph:\n{:?}",
        Dot::with_config(
            &moralized_graph,
            &[Config::EdgeNoLabel, Config::NodeIndexLabel]
        )
    );
}