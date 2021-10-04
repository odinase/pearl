
pub trait NodePotential {
    type Value;

    fn phi(&self, xi: Self::Value) -> f64;
}

pub trait EdgePotential {
    type Value;

    fn psi(&self, xi: Self::Value, xj: Self::Value) -> f64;
}
