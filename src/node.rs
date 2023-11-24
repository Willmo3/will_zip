// A single node, capable of being filtered into a heap.
// Author: Will Morris.
use std::cmp::Ordering;

// A node in this context is a data tuple.
#[derive(PartialEq, Eq, Debug)]
pub struct Node {
    byte: u8,
    count: usize,
}

impl Node {
    pub fn new(byte: u8, count: usize) -> Node {
        Self { byte: byte, count: count }
    }

    // Value accessor needed for sorting normalization
    pub fn byte(&self) -> u8 {
        self.byte.clone()
    }
}

// Explicit ord implementation needed to ensure count considered first.
// default ord implementation would compare based on ordering of struct fields.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
            .then_with(|| self.byte.cmp(&other.byte))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
