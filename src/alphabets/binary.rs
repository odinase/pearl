use super::{Alphabet, Observation};
use crate::markov::potentials::{EdgePotential, NodePotential};
use rand::Rng;
use rand_distr::{Distribution, Uniform};
use std::fmt;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum BinaryAlphabet {
    Zero,
    One,
}

impl fmt::Display for BinaryAlphabet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryAlphabet::Zero => write!(f, "0"),
            BinaryAlphabet::One => write!(f, "1"),
        }
    }
}

impl BinaryAlphabet {
    pub fn random<R: Rng>(mut rng: &mut R) -> Self {
        if Uniform::new(0.0, 1.0).sample(&mut rng) < 0.5 {
            BinaryAlphabet::Zero
        } else {
            BinaryAlphabet::One
        }
    }
}

impl Alphabet for BinaryAlphabet {
    type State = BinaryAlphabet;
    type StateIter = BinaryIter;
    fn states() -> Self::StateIter {
        BinaryIter::new()
    }
    fn size() -> usize {
        2
    }
}

pub struct BinaryIter {
    states: [BinaryAlphabet; 2],
    counter: usize,
}

impl BinaryIter {
    fn new() -> Self {
        BinaryIter {
            states: [BinaryAlphabet::Zero, BinaryAlphabet::One],
            counter: 0,
        }
    }
}

impl Iterator for BinaryIter {
    type Item = BinaryAlphabet;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < 2 {
            let i = self.counter;
            self.counter += 1;
            Some(self.states[i])
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

impl<T: fmt::Display> fmt::Display for Phi<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.observed {
            Observation::Observed(v) => write!(f, "{}", v),
            Observation::Unobserved => write!(f, "???"),
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
    type Value = BinaryAlphabet;

    fn psi(&self, xi: Self::Value, xj: Self::Value) -> f64 {
        let eq = xi == xj;
        self.alpha * (eq as u32 as f64) + (!eq as u32 as f64)
    }
}

impl fmt::Display for Psi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
