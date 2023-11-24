// A single node, capable of being filtered into a heap.
// Author: Will Morris.
use std::cmp::Ordering;

// A node in this context is a data tuple.
#[derive(PartialEq, Eq, Debug)]
pub struct Node {
    value: u8,
    count: usize,
}

impl Node {
    pub fn new(value: u8, count: usize) -> Node {
        Self { count: count, value: value }
    }

    // Value accessor needed for sorting normalization
    pub fn value(&self) -> u8 {
        self.value.clone()
    }
}

// Explicit ord implementation needed to ensure count considered first.
// default ord implementation would compare based on ordering of struct fields.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
            .then_with(|| self.value.cmp(&other.value))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
