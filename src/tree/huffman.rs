use std::collections::{BinaryHeap, HashMap};
use crate::{freq, tree};
use tree::node;

/// Prepare a Huffman tree from a given frequency map.
/// Return the root of the tree if any items or present,
/// Or nothing otherwise.
pub fn prepare_huffman(frequency: &HashMap::<u8, usize>) -> Option<node::Node> {
    // Prepare base heap with all elements sorted by frequency.
    // These are all the leaf nodes.
    let mut heap = frequency.iter().fold(
            BinaryHeap::new(), | mut heap, (byte, count) | {

        let freq = freq::FreqCount::new(*byte, *count);
        heap.push(node::leaf(freq));
        heap
    });

    // Now prepare internal nodes with children.
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let highest = node::internal(Box::from(left), Box::from(right));
        heap.push(highest);
    }

    // The last element in the heap is the root node!
    // Note: if no frequencies supplied, this will be none.
    heap.pop()
}