use std::collections::HashSet;
use std::rc::Rc;

// Generic trait over nodes. Returns value and index of node in graph
pub trait Node<N>
where
    N: Node<N>,
{
    type Value;

    fn index(&self) -> u32;
    fn value(&self) -> Self::Value;
    fn neighbors(&self) -> HashSet<&N>;
}
