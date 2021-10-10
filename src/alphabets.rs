pub mod pollen_allergy;
pub mod binary;


#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Observation<T> {
    Observed(T),
    Unobserved,
}

pub trait Alphabet {
    type State;
    type StateIter: Iterator<Item = Self::State>;

    fn size() -> usize {
        Self::states().count()
    }
    fn states() -> Self::StateIter;
}
