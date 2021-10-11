use crate::markov::potentials::{EdgePotential, NodePotential};
use crate::alphabets::{Alphabet, Observation};
use petgraph::graph::UnGraph;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum GeneAlphabet {
    A,
    B,
}


impl Alphabet for GeneAlphabet {
    type State = GeneAlphabet;
    type StateIter = GeneIter;
    fn states() -> Self::StateIter {
        GeneIter::new()
    }
    fn size() -> usize {
        2
    }
}

pub struct GeneIter {
    genes: [GeneAlphabet; 2],
    counter: usize,
}

impl GeneIter {
    fn new() -> Self {
        GeneIter {
            genes: [GeneAlphabet::A, GeneAlphabet::B],
            counter: 0,
        }
    }
}

impl Iterator for GeneIter {
    type Item = GeneAlphabet;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < 2 {
            let i = self.counter;
            self.counter += 1;
            Some(self.genes[i])
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Phi<T> {
    beta: f64,
    observed: Observation<T>,
}

impl<T> Phi<T> {
    pub fn new_observed(beta: f64, observed_value: T) -> Self {
        Phi {
            beta,
            observed: Observation::Observed(observed_value),
        }
    }

    pub fn new_unobserved(beta: f64) -> Self {
        Phi {
            beta,
            observed: Observation::Unobserved,
        }
    }
}

impl<T: PartialEq> NodePotential for Phi<T> {
    type Value = T;

    fn phi(&self, xi: Self::Value) -> f64 {
        if let Observation::Observed(y) = &self.observed {
            let eq = y == &xi;
            self.beta * (eq as u32 as f64) + (1.0 - self.beta) * (!eq as u32 as f64)
        } else {
            1.0
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Psi {
    alpha: f64,
}

impl Psi {
    pub fn new(alpha: f64) -> Self {
        Psi { alpha }
    }
}

impl EdgePotential for Psi {
    type Value = GeneAlphabet;

    fn psi(&self, xi: Self::Value, xj: Self::Value) -> f64 {
        let eq = xi == xj;
        self.alpha * (eq as u32 as f64) + (!eq as u32 as f64)
    }
}

pub fn load_ungraph_from_file(alpha: f64, beta: f64, filename: &str) -> UnGraph<Phi<GeneAlphabet>, Psi> {
    let file = File::open(filename).unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect(); // BufRead insists on packing strings into Results

    let n: usize = lines[0].parse().unwrap();

    let mut graph = UnGraph::default();

    for node_vals in lines[n..].iter().map(|s| s.split(" ")) {
        let node_potential = {
            match node_vals.last().unwrap().chars().next().unwrap() {
                'A' => Phi::new_observed(beta, GeneAlphabet::A),
                'B' => Phi::new_observed(beta, GeneAlphabet::B),
                '?' => Phi::new_unobserved(beta),
                _ => continue,
            }
        };
        graph.add_node(node_potential);
    }

    for mut edges in lines[1..n]
        .iter()
        .map(|s| s.split(" ").map(|s| s.parse::<u32>().unwrap()))
    {
        graph.add_edge(
            edges.next().unwrap().into(),
            edges.next().unwrap().into(),
            Psi::new(alpha),
        );
    }
    graph
}
