use petgraph::graph::DiGraph;

pub struct BayesNetwork<P> {
    graph: DiGraph<P, ()>,
}