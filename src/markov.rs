// use crate::node::Node;
use itertools::Itertools;
use std::collections::HashMap;
use ndarray::prelude::*;
use petgraph::graph::{NodeIndex, UnGraph};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::marker::PhantomData;
use crate::utils::functions::logsumexp;
use crate::alphabets::Alphabet;

pub mod potentials;
pub mod message_bank;

use self::message_bank::MessageBank;
use self::potentials::{EdgePotential, NodePotential};

pub struct MarkovRandomField<X, NP, EP> {
    pub(crate) graph: UnGraph<NP, EP>,
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

    pub fn belief_propagation(&self) -> Array2<f64> {
        let num_nodes = self.graph.node_count();
        let mut log_messages = MessageBank::new(X::size(), num_nodes, 0.0);//HashMap<(usize, NodeIndex, NodeIndex), f64> = HashMap::new();
        let d = 20; // TODO: Fix this

        let mut log_message_container: Vec<f64> = Vec::with_capacity(X::size());
        for _ in 0..d {
            for n in (0..num_nodes).map(NodeIndex::new) {
                for m in self.graph.neighbors(n) {
                    for (j, xj) in X::states().enumerate() {
                        for (i, xi) in X::states().enumerate() {
                            let log_message_from_neighbors: f64 =  self
                                .graph
                                .neighbors(m) // Loop over neighboring nodes
                                .filter(|&k| k != n) // Exclude node j from the neighboring set
                                .map(|k| *log_messages.message(k, m).eval_state(i)) // Get the value of the message for value xi, from k to i
                                .sum(); // Take the product of all messages
                            let phi = self
                                .node_potential(m)
                                .expect("Invalid node index, but should be valid??");
                            let psi = self.edge_potential(n, m).expect(&format!(
                                "Should be an edge between nodes {:?} and {:?}, but isn't!",
                                n, m
                            ));
                            log_message_container.push(phi.phi(xi).ln() + psi.psi(xi, xj).ln() + log_message_from_neighbors);
                        }
                        *log_messages.message_mut(m, n).eval_state_mut(j) = logsumexp(log_message_container.as_slice());
                        log_message_container.clear();
                    }
                }
            }
        }

        let mut p = Array2::zeros((num_nodes, X::size()));

        for j in (0..num_nodes).map(NodeIndex::new) {
            let phi = self.node_potential(j).unwrap();
            for (i, xi) in X::states().enumerate() {
                let incoming_log_messages: f64 = self
                    .graph
                    .neighbors(j)
                    .map(|k| *log_messages.message(k, j).eval_state(i))
                    .sum();
                p[(j.index(), i)] = phi.phi(xi).ln() + incoming_log_messages;
            }
            // Normalize with total sum
            let s = logsumexp(p.row(j.index()).as_slice().unwrap());
            for pp in p.row_mut(j.index()) {
                *pp = (*pp - s).exp();
            }
        }

        p
    }

    pub fn min_sum(&self) -> Vec<X::State> {

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
