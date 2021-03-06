use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::cell::RefCell;
use petgraph::graph::NodeIndex;


pub struct MessageRef<'a> {
    message: &'a [f64]
}

impl<'a> MessageRef<'a> {
    fn from_slice(message: &'a [f64]) -> Self {
        MessageRef {
            message
        }
    }

    pub fn eval_state(&self, state: usize) -> &f64 {
        &self.message[state]
    }
}


pub struct MessageRefMut<'a> {
    message: &'a mut [f64]
}


impl<'a> MessageRefMut<'a> {
    fn from_slice_mut(message: &'a mut [f64]) -> Self {
        MessageRefMut {
            message
        }
    }

    pub fn eval_state_mut(&mut self, state: usize) -> &mut f64 {
        &mut self.message[state]
    }
}


pub struct MessageBank {
    indices: RefCell<HashMap<(NodeIndex, NodeIndex), usize>>,
    bank: Vec<f64>,
    last_added_idx: RefCell<Option<usize>>,
    num_states: usize,
}

impl MessageBank {
    pub fn new(num_states: usize, num_edges: usize, init_message_val: f64) -> Self {
        MessageBank {
        indices: RefCell::new(HashMap::new()),
        bank: vec![init_message_val; num_states*num_edges*2],
        last_added_idx: RefCell::new(None),
        num_states,
    }
}

    pub fn message(&self, from: NodeIndex, to: NodeIndex) -> MessageRef {
        if self.indices.borrow().contains_key(&(from, to)) {
            let i = self.indices.borrow()[&(from, to)];
            MessageRef::from_slice(&self.bank[i..i+self.num_states])
        } else {
            let mut last_idx = self.last_added_idx.borrow_mut();
            match *last_idx {
                Some(idx) => {
                    let new_idx = idx + self.num_states;
                    *last_idx = Some(new_idx);
                    self.indices.borrow_mut().insert((from, to), new_idx);
                    MessageRef::from_slice(&self.bank[new_idx..new_idx + self.num_states])
                },
                None => {
                    *last_idx = Some(0);
                    self.indices.borrow_mut().insert((from, to), 0);
                    MessageRef::from_slice(&self.bank[0..self.num_states])
                }
            }
        }
}

pub fn message_mut(&mut self, from: NodeIndex, to: NodeIndex) -> MessageRefMut {
    if self.indices.borrow().contains_key(&(from, to)) {
        let i = self.indices.borrow()[&(from, to)];
        MessageRefMut::from_slice_mut(&mut self.bank[i..i+self.num_states])
    } else {
        let mut last_idx = self.last_added_idx.borrow_mut();
        match *last_idx {
            Some(idx) => {
                let new_idx = idx + self.num_states;
                *last_idx = Some(new_idx);
                self.indices.borrow_mut().insert((from, to), new_idx);
                MessageRefMut::from_slice_mut(&mut self.bank[new_idx..new_idx + self.num_states])
            },
            None => {
                *last_idx = Some(0);
                self.indices.borrow_mut().insert((from, to), 0);
                MessageRefMut::from_slice_mut(&mut self.bank[0..self.num_states])
            }
        }
    }
}
}

impl Index<(usize, NodeIndex, NodeIndex)> for MessageBank {
    type Output = f64;

    fn index(&self, idx: (usize, NodeIndex, NodeIndex)) -> &Self::Output {
        let (state, i, j) = idx;
        if self.indices.borrow().contains_key(&(i, j)) {
            &self.bank[state + self.indices.borrow()[&(i, j)]]
        } else {
            let mut last_idx = self.last_added_idx.borrow_mut();
            match *last_idx {
                Some(idx) => {
                    let new_idx = idx + self.num_states;
                    *last_idx = Some(new_idx);
                    self.indices.borrow_mut().insert((i, j), new_idx);
                    &self.bank[new_idx + state]
                },
                None => {
                    *last_idx = Some(0);
                    self.indices.borrow_mut().insert((i, j), 0);
                    &self.bank[state]
                }
            }
        }
    }
}


impl IndexMut<(usize, NodeIndex, NodeIndex)> for MessageBank {
    fn index_mut(&mut self, idx: (usize, NodeIndex, NodeIndex)) -> &mut Self::Output {
        let (state, i, j) = idx;
        if self.indices.borrow().contains_key(&(i, j)) {
            &mut self.bank[state + self.indices.borrow()[&(i, j)]]
        } else {
            let mut last_idx = self.last_added_idx.borrow_mut();
            match *last_idx {
                Some(idx) => {
                    let new_idx = idx + self.num_states;
                    *last_idx = Some(new_idx);
                    self.indices.borrow_mut().insert((i, j), new_idx);
                    &mut self.bank[new_idx + state]
                },
                None => {
                    *last_idx = Some(0);
                    self.indices.borrow_mut().insert((i, j), 0);
                    &mut self.bank[state]
                }
            }
        }
    }
}