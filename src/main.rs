use itertools::Itertools;
use pearl::markov::MarkovRandomField;
use pearl::alphabets::pollen_allergy::{load_ungraph_from_file, GeneAlphabet};
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, UnGraph};
use petgraph::visit::Dfs;
use petgraph::Direction;
use std::time::{Duration, Instant};
use pearl::utils::{testing, printing};
use rand::{rngs::StdRng, SeedableRng};


fn moralize<N, E: std::default::Default>(mut graph: DiGraph<N, E>) -> UnGraph<N, E> {
    for i in graph.node_indices() {
        let num_parents = graph.neighbors_directed(i, Direction::Incoming).count();
        if num_parents > 1 {
            // The node has parents that needs to be moralized
            let new_edges: Vec<_> = graph
                .neighbors_directed(i, Direction::Incoming) // All parents
                .combinations(2) // Create pairwise parents that need moralizing
                .filter_map(|v| {
                    if !graph.contains_edge(v[0], v[1]) && !graph.contains_edge(v[1], v[0])
                    /* Filter out already connected parents */
                    {
                        Some((v[0], v[1])) // Repack into tuple
                    } else {
                        None
                    }
                })
                .collect();
            let new_parents_iter = new_edges.into_iter();
            for (from, to) in new_parents_iter {
                graph.update_edge(from, to, E::default());
            }
        }
    }
    graph.into_edge_type()
}

fn main() {
    // let graph = load_ungraph_from_file(2.0, 1.0, "./data/family-tree.txt");
    // // println!(
    // //     "{:?}",
    // //     Dot::with_config(&graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel])
    // // );
    let mut rng: StdRng = SeedableRng::seed_from_u64(12345);
    let num_nodes = 1000;
    let mrf = testing::random_binary_mrf(num_nodes, &mut rng);
    // printing::print_mrf_to_file_with_config("mrf.txt", &mrf, &[Config::EdgeNoLabel]);

    // let mut times = Vec::new();
    // for _ in 0..100 {
    //     let start = Instant::now();
    //     mrf.belief_propagation();
    //     let stop = Instant::now();
    //     let dt = stop - start;
    //     times.push(dt);
    // }
    // let average: f64 = times
    //     .iter()
    //     .map(|t| t.as_nanos() as f64 / 1000.0)
    //     .sum::<f64>()
    //     / times.len() as f64;
    // let std: f64 = (times
    //     .iter()
    //     .map(|t| (t.as_nanos() as f64 / 1000.0))
    //     .map(|t| (t - average) * (t - average))
    //     .sum::<f64>()
    //     / times.len() as f64)
    //     .sqrt();

    // println!("Spent average {} us +- {}", average, std);

    let p = mrf.belief_propagation();
    // for p in p.rows() {
    //     println!("{}", p);
    // }

    // let mut dfs = Dfs::new(&graph, 0.into());
    // while let Some(nx) = dfs.next(&graph) {

    // }

    // let digraph: DiGraph<i32, ()> =
    //     DiGraph::from_edges(&[(0, 3), (2, 3), (1, 4), (3, 4), (1, 5), (4, 5), (0, 5)]);

    // println!(
    //     "Directed graph:\n{:?}",
    //     Dot::with_config(&digraph, &[Config::EdgeNoLabel, Config::NodeIndexLabel])
    // );

    // let moralized_graph = moralize(digraph);
    // println!(
    //     "Moralized graph:\n{:?}",
    //     Dot::with_config(
    //         &moralized_graph,
    //         &[Config::EdgeNoLabel, Config::NodeIndexLabel]
    //     )
    // );
}
