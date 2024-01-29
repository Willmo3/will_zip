use std::collections::HashMap;

// Order the bytes in a stream based on how often they appear.
// Needed for compression
// Author: Will Morris

// Generate a frequency of all the bytes in a file.
pub fn gen_frequency(bytes: &[u8]) -> HashMap<u8, u64> {
    bytes.iter().fold(HashMap::<u8, u64>::new(), | mut map, curr | {
        if !map.contains_key(curr) {
            map.insert(*curr, 0);
        }
        map.insert(*curr, map.get(curr).unwrap() + 1);
        map
    })
}