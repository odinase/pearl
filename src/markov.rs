// use crate::node::Node;
use itertools::Itertools;
use std::collections::HashMap;
use ndarray::prelude::*;
use petgraph::graph::{NodeIndex, UnGraph};
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
        MarkovRandomField {
            graph,
            _alphabet: PhantomData,
        }
    }

    pub fn sum_product(&self) -> Array2<f64> {
        let num_nodes = self.graph.node_count();
        let mut messages: HashMap<(usize, NodeIndex, NodeIndex), f64> = HashMap::new();
        let d = 20; // TODO: Fix this

        for _ in 0..d {
            for n in (0..num_nodes).map(NodeIndex::new) {
                for m in self.graph.neighbors(n) {
                    for (j, xj) in X::states().enumerate() {
                        let mut message = 0.0f64;
                        for (i, xi) in X::states().enumerate() {
                            let message_from_neighbors: f64 =  self
                                .graph
                                .neighbors(m) // Loop over neighboring nodes
                                .filter(|&k| k != n) // Exclude node j from the neighboring set
                                .map(|k| *messages.entry((i, k, m)).or_insert(1.0)) // Get the value of the message for value xi, from k to i
                                .product(); // Take the product of all messages
                            let phi = self
                                .node_potential(m)
                                .expect("Invalid node index, but should be valid??");
                            let psi = self.edge_potential(n, m).expect(&format!(
                                "Should be an edge between nodes {:?} and {:?}, but isn't!",
                                n, m
                            ));
                            message += phi.phi(xi) * psi.psi(xi, xj) * message_from_neighbors;
                        }
                        messages.insert((j, m, n), message);
                    }
                }
            }
        }

        let mut p = Array2::zeros((num_nodes, X::size()));

        for j in (0..num_nodes).map(NodeIndex::new) {
            let mut sum = 0.0;
            let phi = self.node_potential(j).unwrap();
            for (i, xi) in X::states().enumerate() {
                let incoming_messages: f64 = self
                    .graph
                    .neighbors(j)
                    .map(|k| messages[&(i, k, j)])
                    .product();
                p[(j.index(), i)] = phi.phi(xi) * incoming_messages;
                sum += p[(j.index(), i)]
            }
            // Normalize with total sum
            for pp in p.row_mut(j.index()) {
                *pp = *pp / sum;
            }
        }

        p
    }

    fn node_potential(&self, index: NodeIndex) -> Option<&NP> {
        self.graph.node_weight(index)
    }

    fn edge_potential(&self, node1: NodeIndex, node2: NodeIndex) -> Option<&EP> {
        self.graph.edge_weight(
            self.graph
                .find_edge(node1, node2)?,
        )
    }
}
