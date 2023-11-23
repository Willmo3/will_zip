// A single node, capable of being filtered into a heap.
// Author: Will Morris.

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Node {
    value: u8,
    count: usize,
}

impl Node {
    pub fn new(value: u8, count: usize) -> Node {
        Self { count: count, value: value }
    }
}
