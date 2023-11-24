use std::collections::BinaryHeap;
use std::collections::HashMap;
use crate::freq;

// Generate a binary heap of the encoding.
pub fn gen_encoding (table: &HashMap::<u8, usize>) -> BinaryHeap<freq::FreqCount> {
    // Need to vectorize the table.
    table.iter().fold(BinaryHeap::new(), | mut heap, (byte, count) | {
        heap.push(freq::FreqCount::new(byte.clone(), count.clone()));
        heap
    })
}
