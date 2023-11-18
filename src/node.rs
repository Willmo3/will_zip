use std::cmp::Ordering;

// A single node, capable of being filtered into a heap.
// Author: Will Morris.

pub struct Node {
    count: usize,
    value: u8,
}

// Ordered nature of node is key of binary heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Ties must be broken or decompression is nondeterministic!
        other.count.cmp(&self.count)
            .then_with(|| self.value.cmp(&other.value))    
    }
}
