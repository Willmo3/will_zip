// A single node, capable of being filtered into a heap.
// Author: Will Morris.

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    count:  usize,
    value: u8,
}
