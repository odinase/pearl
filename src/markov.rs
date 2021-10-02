use crate::node::Node;
use petgraph::graph::UnGraph;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use itertools::Itertools;


#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Value<X> {
    Observed(X),
    Unobserved,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Variable<X> {
    index: u32,
    value: Value<X>,
}

impl<X> Variable<X> {
    pub fn new(index: u32, value: Value<X>) -> Self {
        Variable {
            index,
            value
        }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn value(&self) -> Value<X> {
        self.value
    }

    pub fn is_observed(&self) -> bool {
        match self.value {
            Value::Observed(_) => true,
            Value::Unobserved => false,
        }
    }

    pub fn is_unobserved(&self) -> bool {
        !self.is_observed()
    }
}


pub trait NodePotential<T> {
    fn phi(&self, xi: Variable<T>, yi: Option<Variable<T>>) -> f64;
}

pub trait EdgePotential<T> {
    fn psi(&self, xi: Variable<T>, xj: Variable<T>) -> f64;
}

pub struct MarkovRandomField<X, NP, EP> {
    graph: UnGraph<Variable<X>, ()>,
    np: NP,
    ep: EP,
}

impl<T, NP, EP> MarkovRandomField<T, NP, EP>
where
    NP: NodePotential<T>,
    EP: EdgePotential<T>,
{
    pub fn new(graph: UnGraph<Variable<T>, ()>, np: NP, ep: EP) -> Self {
        MarkovRandomField {
            graph,
            np,
            ep,
        }
    }    
}
