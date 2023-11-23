use std::collections::BinaryHeap;
use std::collections::HashMap;
use crate::node;

// Generate a binary heap of the encoding.
pub fn gen_encoding (table: &HashMap::<u8, usize>) -> BinaryHeap<node::Node> {
    // Need to vectorize the table.
    let heap: BinaryHeap<node::Node> = table.iter().fold(BinaryHeap::new(), | mut heap, (byte, count) | {
        heap.push(node::Node::new(byte.clone(), count.clone()));
        heap
    });
    heap
}
