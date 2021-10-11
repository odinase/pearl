// use crate::node::Node;
use crate::alphabets::Alphabet;
use crate::utils::functions::logsumexp;
use argmax::Argmax;
use itertools::Itertools;
use ndarray::prelude::*;
use petgraph::graph::{node_index as nidx, NodeIndex, UnGraph};
use petgraph::visit::{depth_first_search, Control, DfsEvent};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::marker::PhantomData;

pub mod message_bank;
pub mod potentials;

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
        let mut log_messages = MessageBank::new(X::size(), num_nodes, 0.0); //HashMap<(usize, NodeIndex, NodeIndex), f64> = HashMap::new();
        let d = 20; // TODO: Fix this

        let mut log_message_container: Vec<f64> = Vec::with_capacity(X::size());
        for _ in 0..d {
            for n in (0..num_nodes).map(nidx) {
                for m in self.graph.neighbors(n) {
                    for (j, xj) in X::states().enumerate() {
                        for (i, xi) in X::states().enumerate() {
                            let log_message_from_neighbors: f64 = self
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
                            log_message_container.push(
                                phi.phi(xi).ln()
                                    + psi.psi(xi, xj).ln()
                                    + log_message_from_neighbors,
                            );
                        }
                        *log_messages.message_mut(m, n).eval_state_mut(j) =
                            logsumexp(log_message_container.as_slice());
                        log_message_container.clear();
                    }
                }
            }
        }

        let mut p = Array2::zeros((num_nodes, X::size()));

        for j in (0..num_nodes).map(nidx) {
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

    pub fn min_sum(&self) -> HashMap<NodeIndex, X::State> {
        let num_nodes = self.graph.node_count();
        let num_edges = self.graph.edge_count();
        let mut log_messages = MessageBank::new(X::size(), num_nodes, 0.0); //HashMap<(usize, NodeIndex, NodeIndex), f64> = HashMap::new();
        let mut backpointers: HashMap<(NodeIndex, NodeIndex), Vec<Option<usize>>> =
            HashMap::with_capacity(2 * num_edges); //= MessageBank::new(X::size(), num_nodes, 0.0);//HashMap<(usize, NodeIndex, NodeIndex), f64> = HashMap::new();

        let d = 20; // TODO: Fix this
        let mut log_message_container = Vec::with_capacity(X::size());
        for _ in 0..d {
            for n in (0..num_nodes).map(NodeIndex::new) {
                for m in self.graph.neighbors(n) {
                    for (j, xj) in X::states().enumerate() {
                        for (i, xi) in X::states().enumerate() {
                            let log_message_from_neighbors: f64 = self
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
                            log_message_container.push(
                                phi.phi(xi).ln()
                                    + psi.psi(xi, xj).ln()
                                    + log_message_from_neighbors,
                            );
                        }
                        let (max_index, max_val) = log_message_container
                            .iter()
                            .argmax()
                            .expect("Log message container empty??");
                        *log_messages.message_mut(m, n).eval_state_mut(j) = *max_val;
                        backpointers.entry((m, n)).or_insert(vec![None; X::size()])[j] =
                            Some(max_index);
                        log_message_container.clear();
                    }
                }
            }
        }

        // Traverse from node 0 (arbitraty) and propagate outwards
        let mut map_estimates: HashMap<NodeIndex, X::State> = HashMap::with_capacity(num_nodes);

        let start = nidx(0);
        let phi_start = self
            .node_potential(start)
            .expect("Node 0 is not in graph??");
        let mut max_marginal = Vec::with_capacity(X::size());

        for (i, xi) in X::states().enumerate() {
            let log_message_sum: f64 = self
                .graph
                .neighbors(start)
                .map(|j| *log_messages.message(start, j).eval_state(i))
                .sum();
            max_marginal.push(phi_start.phi(xi) + log_message_sum);
        }

        let (map_index, _) = max_marginal.iter().argmax().unwrap();

        let map_state = X::try_from_index(map_index).expect("Should be valid index!!");
        map_estimates.insert(start, map_state);

        let mut backpoint_containter = Vec::with_capacity(X::size());
        depth_first_search(&self.graph, Some(start), |event| {
            if let DfsEvent::TreeEdge(from, to) = event {
                let xi = map_estimates[&from]; // I guess we have already recorded the max state of from node when doing dfs???

                for (j, xj) in X::states().enumerate() {
                    let log_neighboring_messages_sum: f64 = self
                    .graph
                    .neighbors(from)
                    .filter(|&k| k != to)
                    .map(|j| *log_messages.message(start, j).eval_state(X::to_index(xi)))
                    .sum();
                    backpoint_containter.
                }

            }
            Control::<()>::Continue
        });

        map_estimates
    }

    fn node_potential(&self, index: NodeIndex) -> Option<&NP> {
        self.graph.node_weight(index)
    }

    fn edge_potential(&self, node1: NodeIndex, node2: NodeIndex) -> Option<&EP> {
        self.graph.edge_weight(self.graph.find_edge(node1, node2)?)
    }
}
