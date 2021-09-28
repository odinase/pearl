use petgraph::graph::UnGraph;
use crate::node::Node;

pub trait NodePotential {
    type Variable;

    fn phi(xi: Self::Variable) -> f64;
}

pub trait EdgePotential {
    type Variable;

    fn psi(xi: Self::Variable, xj: Self::Variable) -> f64;
}

pub struct MarkovRandomField<P, NP, EP> {
    graph: UnGraph<P, ()>,
    np: NP,
    ep: EP,
}



impl<P, NP, EP> MarkovRandomField<P, NP, EP>
where
P: Node,
    NP: NodePotential,
    NP::Variable: Node,
    EP: EdgePotential,
    EP::Variable: Node,
{
    pub fn new(np: NP, ep: EP) -> Self {
        MarkovRandomField {
            graph: UnGraph::default(),
            np,
            ep
        }
    }

    
}
