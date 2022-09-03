// use crate::node::Node;
use itertools::Itertools;
use petgraph::data::Build;
use std::collections::HashMap;
use ndarray::prelude::*;
use ndarray::arr2;

use petgraph::graph::{NodeIndex, UnGraph};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::marker::PhantomData;
use crate::utils;
use crate::utils::logsumexp;
use petgraph::dot::{Config, Dot};


use std::collections::{
    hash_map::DefaultHasher,
    HashSet
};
use std::hash::{Hash, Hasher};
use std::fmt;

use std::convert::From;


type Messages = HashMap<(NodeIndex, NodeIndex), Vec<f64>>;



fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Eq)]
pub struct Key(u64);

impl Key {
    pub fn symbol(c: char, id: u32) -> Key {
        let k: u64 = ((c as u64) << 32) | (id as u64);
        Key(k)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}{}", (self.0 >> 32) as u8 as char, self.0 & 0xFFFF)
    }
}

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use Display implementation
        write!(f, "{}", self)
    }
}


#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Eq)]
pub struct Variable {
    key: Key,
    cardinality: u32,
}


impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.key)
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use Display implementation
        write!(f, "{}", self.key)
    }
}

impl Variable {
    pub fn new(key: Key, cardinality: u32) -> Variable {
        Variable { key , cardinality }
    }
}

#[derive(Clone)]
pub struct Factor {
    vars: Vec<Variable>,
    table: ArrayD<f64>,
}

impl Factor {
    pub fn new(vars: Vec<Variable>, table: ArrayD<f64>) -> Factor {
        Factor { vars, table }
    }
}

impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.vars)
    }
}


impl fmt::Debug for Factor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use Display implementation
        write!(f, "{}", self)
    }
}

// Factor graph node. Use enum for fast code B-)
#[derive(Debug)]
enum Node {
    Variable(Variable),
    Factor(Factor),
}

impl Node {
    pub fn to_variable(&self) -> &Variable {
        match self {
            Self::Variable(v) => v,
            Self::Factor(_) => panic!("tried to cast factor to variable!"),
        }
    }

    pub fn to_factor(&self) -> &Factor {
        match self {
            Self::Variable(_) => panic!("tried to cast variable to factor!"),
            Self::Factor(f) => f,
        }
    }
}


pub struct FactorGraph {
    graph: UnGraph<Node, ()>,
    variables: HashMap<Variable, NodeIndex>
}

impl FactorGraph {
    pub fn new() -> FactorGraph {
        FactorGraph { graph: UnGraph::default(), variables: HashMap::new() }
    }

    // We don't make this function public as it doesn't make sense to only add a variable. Every variable needs at least one factor associated with it.
    fn add_variable(&mut self, var: Variable) -> NodeIndex {
        // The variable is already added to the factor graph, return index immediately
        if let Some(&n) = self.variables.get(&var) {
            n
        // We didn't find the variable, so add it and return the new variable
        } else {
            let n = self.graph.add_node(Node::Variable(var));
            println!("Adding new variable {}", var.key);
            self.variables.insert(var, n);
            n
        }
    }

    pub fn add_factor(&mut self, factor: Factor) {
        // TODO(odin): Should be possible to avoid the clone here...
        let vars = factor.vars.clone();

        // Add factor node
        let f = self.graph.add_node(Node::Factor(factor));

        // Connect factor to all variables.
        for var in vars {
            // Add variable to graph if it doesn't exist there already
            let v = self.add_variable(var);
            self.graph.add_edge(v, f, ());
        }
    }

    pub fn print(&self) {
            println!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
    }

    fn messages_converged(prev_messages: &Messages, messages: &Messages, kl_threshold: f64) -> bool {
        prev_messages.values().zip(messages.values()).all(|(prev_marginal, marginal)| utils::kl_divergence(&prev_marginal, &marginal).unwrap() < kl_threshold)
    }

    fn update_message_variable(&self, prev_messages: &Messages, mut messages: Messages) -> Messages {
        // Remember to only update messages, but use data in prev_messages
        


        todo!()
    }

    fn update_message_factor(&self, prev_messages: &Messages, mut messages: Messages) -> Messages {
        todo!()
    }

    // Run loopy belief propagation and return map over all variables and corresponding marginal
    pub fn loopy_belief_propagation(&self, max_iterations: usize, convergence_threshold: f64) -> HashMap<Variable, Vec<f64>> {
        let mut marginals = HashMap::new();
        let mut messages: HashMap<(NodeIndex, NodeIndex), Vec<f64>> = HashMap::new(); // Map from pair of nodes (variable and factor) to vector of values proportional to the marginal

        // Based on the equations
        // m_{a\to i}(x_i) &= \sum_{b\in \mathsf{N}(a)\setminus\set{i}} f_a(x_{\mathsf{N}(i)})\prod_{j\in \mathsf{N}(a)\setminus\set{i}}m_{j\to a}(x_j) \\
        // m_{i\to a}(x_i) &= \prod_{b\in \mathsf{N}(i)\setminus\set{a}}m_{b\to i}(x_j)

        // We should probably check for convergence, KL-divergence?
        let converged = false;
        let mut num_iters = 0;

        while !converged && num_iters < max_iterations {
            // We need the previous messages
            let mut prev_messages = messages.clone();

            // Loop over all nodes
            for node in self.graph.node_indices() {
                match &self.graph[node] {
                    // Here we need to only update messages and keep prev_messages constants
                    Node::Variable(v) => messages = self.update_message_variable(&prev_messages, messages),
                    Node::Factor(f) => messages = self.update_message_factor(&prev_messages, messages)
                }
            }

            num_iters += 1;
        }

        marginals
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_factor_graph() {
        let mut fg = FactorGraph::new();

        let x0 = Variable::new(Key::symbol('x', 0), 2);
        let x1 = Variable::new(Key::symbol('x', 1), 2);
        
        let table = ndarray::arr2(&[
            [0.1, 0.3],
            [0.4, 0.2]
        ]).into_dyn();
        let vars = vec![x0, x1];
        let f = Factor::new(vars, table);

        fg.add_factor(f);

        fg.print();
    }
}