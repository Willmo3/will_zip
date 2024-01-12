use std::collections::HashMap;
use crate::ordering::byteordering::ByteOrdering;

// Order the bytes in a stream based on how often they appear.
// Needed for compression
// Author: Will Morris

// Generating an ordering is comprised of two steps:
// 1. Calculating the frequency of all bytes in the files.
// 2. Ordering those bytes from 255-0 in terms of precedence.
pub fn gen_ordering(bytes: &[u8]) -> HashMap<u8, u8> {
    let frequency = gen_frequency(bytes);
    normalize(&frequency)
}

// Create a hashmap of frequencies in a byte map.
fn gen_frequency(bytes: &[u8]) -> HashMap::<u8, usize> {
    bytes.iter().fold(HashMap::<u8, usize>::new(), | mut map, curr | {
        if !map.contains_key(curr) {
            map.insert(*curr, 0);
        }
        map.insert(*curr, map.get(curr).unwrap() + 1);
        map
    })
}

// Given a map of frequencies, "normalize" them so that they are ordered 0-255.
// Note that this WILL work, as there are only 255 u8 values.
// The reason we're doing this is that storing a hashmap of u8-u8 is smaller.
// If we do this, we can reliably guarantee that each encoding is a single byte pair.
fn normalize(original: &HashMap::<u8, usize>) -> HashMap<u8, u8> {
   let mut storage_vec = original.iter().fold(Vec::new(),
                                              | mut vec, (byte, count) | {
       vec.push(ByteOrdering::new(*byte, *count));
       vec
   });
   storage_vec.sort();

   let mut retmap: HashMap::<u8, u8> = HashMap::new();
   for i in 0..storage_vec.len() {
       retmap.insert(storage_vec[i].byte(), i as u8);
   }
   retmap
}




