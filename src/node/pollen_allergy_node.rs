use petgraph::graph::UnGraph;
use std::fs::File;
use super::Node;
use std::io::{self, BufRead, BufReader};
use crate::markov::{NodePotential, EdgePotential, Variable};

pub struct Phi {
    pub beta: f64,
}

impl<T: Copy + PartialEq> NodePotential<T> for Phi {
    fn phi(&self, xi: Variable<T>, yi: Option<Variable<T>>) -> f64 {
        if let Some(yi) = yi {
            if xi.is_observed() {
                let eq = yi.value() == xi.value();
                return self.beta*(eq as u32 as f64) + (1.0 - self.beta)*(!eq as u32 as f64) 
            }
        }
        1.0
    }
}

pub struct Psi {
    pub alpha: f64
}

impl<T: PartialEq> EdgePotential<T> for Psi {
    fn psi(&self, xi: Variable<T>, xj: Variable<T>) -> f64 {
        let eq = xi.value() == xj.value();
        self.alpha*(eq as u32 as f64) + (1.0 - self.alpha)*(!eq as u32 as f64)
    }
}


pub fn load_ungraph_from_file(filename: &str) -> UnGraph<Variable<char>, ()> {
    let file = File::open(filename).unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect(); // BufRead insists on packing strings into Results

    let n: usize = lines[0].parse().unwrap();

    let mut graph = UnGraph::default();

    for mut node_vals in lines[n..].iter().map(|s| s.split(" ")) {
        let node = Variable::new(node_vals.next().unwrap().parse().unwrap(), Value::from(node_vals.next().unwrap().chars().next().unwrap()));
        graph.add_node(node);        
    }

    for mut edges in lines[1..n].iter().map(|s| s.split(" ").map(|s| s.parse::<u32>().unwrap())) {
        graph.add_edge(edges.next().unwrap().into(), edges.next().unwrap().into(), ());
    }

    graph
}
