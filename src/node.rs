use std::cmp::Ordering;

// A single node, capable of being filtered into a heap.
// Author: Will Morris.

pub struct Node {
    count: usize,
    value: u8,
}

// Ordered nature of node is key of binary heap.
// Explicit implementation needed to handle ties.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Ties must be broken or decompression is nondeterministic!
        &self.count.cmp(other.count)
            .then_with(|| self.value.cmp(&other.value))    
    }
}
