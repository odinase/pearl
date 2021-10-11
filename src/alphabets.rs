pub mod pollen_allergy;
pub mod binary;


#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Observation<T> {
    Observed(T),
    Unobserved,
}

pub trait Alphabet {
    type State: PartialEq;
    type StateIter: Iterator<Item = Self::State>;

    fn size() -> usize {
        Self::states().count()
    }
    fn states() -> Self::StateIter;
    fn try_from_index(index: usize) -> Option<Self::State> {
        Self::states()
        .enumerate()
        .find_map(|(i, x)| if i == index { Some(x) } else { None })
    }
    fn to_index(state: Self::State) -> usize {
        Self::states()
        .enumerate()
        .find_map(|(i, x)| if x == state { Some(i) } else { None }).unwrap() // This will never fail
    }
}
