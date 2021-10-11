use std::fs::File;
use std::io::Write;
use petgraph::dot::{Config, Dot};
use petgraph::graph::UnGraph;
use crate::markov::MarkovRandomField;


pub fn print_graph_to_file(filename: &str, tree: &UnGraph<(), ()>) {
    let mut output = File::create(filename).unwrap();
    write!(
        output,
        "{:?}",
        Dot::with_config(&tree, &[Config::EdgeNoLabel, Config::NodeIndexLabel])
    );
}


pub fn print_graph_to_file_with_config(filename: &str, tree: &UnGraph<(), ()>, config: &[Config]) {
    let mut output = File::create(filename).unwrap();
    write!(
        output,
        "{:?}",
        Dot::with_config(&tree, config)
    );
}


pub fn print_mrf_to_file<X, NP: std::fmt::Debug, EP: std::fmt::Debug>(filename: &str, mrf: &MarkovRandomField<X, NP, EP>) {
    let mut output = File::create(filename).unwrap();
    write!(
        output,
        "{:?}",
        Dot::with_config(&mrf.graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel])
    );
}


pub fn print_mrf_to_file_with_config<X, NP: std::fmt::Display, EP: std::fmt::Display>(filename: &str, mrf: &MarkovRandomField<X, NP, EP>, config: &[Config]) {
    let mut output = File::create(filename).unwrap();
    write!(
        output,
        "{}",
        Dot::with_config(&mrf.graph, config)
    );
}