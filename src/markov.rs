use petgraph::graph::UnGraph;

pub struct MarkovRandomField<P> {
    graph: UnGraph<P, ()>,
}