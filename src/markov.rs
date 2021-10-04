// use crate::node::Node;
use itertools::Itertools;
use ndarray::prelude::*;
use petgraph::graph::{UnGraph, NodeIndex};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::marker::PhantomData;

pub mod potentials;
use self::potentials::{EdgePotential, NodePotential};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Observation<T> {
    Observed(T),
    Unobserved,
}

pub trait Alphabet {
    type State;
    type StateIter: Iterator<Item = Self::State>;

    fn size() -> usize {
        Self::states().count()
    }
    fn states() -> Self::StateIter;
}

pub struct MarkovRandomField<X, NP, EP> {
    graph: UnGraph<NP, EP>,
    _alphabet: PhantomData<X>,
}

impl<X, NP, EP> MarkovRandomField<X, NP, EP>
where
X: Alphabet,
X::State: Copy,
    NP: NodePotential<Value = X::State>,
    EP: EdgePotential<Value = X::State>,
{
    pub fn new(graph: UnGraph<NP, EP>) -> Self {
        MarkovRandomField { graph, _alphabet: PhantomData }
    }

    pub fn sum_product(&self) -> Array2<f64> {
        let num_nodes = self.graph.node_count();
        let mut M: Array3<f64> = Array::ones((X::size(), num_nodes, num_nodes));
        let d = 20; // TODO: Fix this

        for _ in 0..d {
            for j in 0..num_nodes {
                for i in self.graph.neighbors((j as u32).into()).map(|i| i.index()) {
                    for (r, xj) in X::states().enumerate() {
                        let mut m = 0.0f64;
                        for (s, xi) in X::states().enumerate() {
                            let mut mm = 1.0f64;
                            for k in self.graph.neighbors(NodeIndex::new(i)).filter(|&k| k.index() != j).map(|k| k.index()) {
                                mm *= M[(s, k, i)]
                            }
                            let phi = self.node_potential(i).expect("Invalid node index, but should be valid??");
                            let psi = self.edge_potential(j, i).expect(&format!("Should be an edge between nodes {} and {}, but isn't!", j, i));
                            m += phi.phi(xi)*psi.psi(xi, xj)*mm;
                        }
                        M[(r, i, j)] = m;
                    }
                }
            }
        }

        let mut p = Array2::zeros((num_nodes, X::size()));

        for i in 0..num_nodes {
            let phi = self.node_potential(i).unwrap();
            for (j, xi) in X::states().enumerate() {
                let m: f64 = self.graph.neighbors(NodeIndex::new(i)).map(|k| M[(j, k.index(), i)]).product();
                p[(i, j)] = phi.phi(xi)*m;
            }
        }

        p = &p/p.sum_axis(Axis(1)).insert_axis(Axis(1));

        p
    }


    fn node_potential(&self, index: usize) -> Option<&NP> {
        self.graph.node_weight(NodeIndex::new(index))
    }

    fn edge_potential(&self, node1: usize, node2: usize) -> Option<&EP> {
        self.graph.edge_weight(self.graph.find_edge(NodeIndex::new(node1), NodeIndex::new(node2))?)
    }
}
