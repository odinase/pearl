use petgraph::graph::DiGraph;

pub struct BayesNetwork<P> {
    graph: DiGraph<P, ()>,
}

impl<P> BayesNetwork<P> {
    pub fn new() -> Self {
        BayesNetwork {
            graph: DiGraph::new()
        }
    }
} 