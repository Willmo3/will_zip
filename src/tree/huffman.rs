use std::collections::{BinaryHeap, HashMap};
use crate::ordering::ordering::ByteOrdering;
use crate::tree::node::{internal, leaf, Node};

/// Prepare a Huffman tree from a given frequency map.
/// Return the root of the tree if any items or present,
/// Or nothing otherwise.
/// NOTE that this expects a normalized frequency -- so will only take in u8!
pub fn prepare_huffman(ordering: &HashMap::<u8, u8>) -> Option<Node> {
    // Prepare base heap with all elements sorted by frequency.
    // These are all the leaf nodes.
    let mut heap = ordering.iter().fold(
            BinaryHeap::new(), | mut heap, (byte, count) | {

        let freq = ByteOrdering::new(*byte, *count as usize);
        heap.push(leaf(freq));
        heap
    });

    // Now prepare internal nodes with children.
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let highest = internal(Box::from(left), Box::from(right));
        heap.push(highest);
    }

    // The last element in the heap is the root node!
    // Note: if no frequencies supplied, this will be none.
    heap.pop()
}