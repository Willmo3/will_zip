use std::collections::HashMap;
use std::cmp::Ordering;

// Create a hashmap of frequencies in a byte map.
pub fn gen_frequency(bytes: &Vec<u8>) -> HashMap::<u8, usize> {
    bytes.iter().fold(HashMap::<u8, usize>::new(), | mut map, curr | {
        if !map.contains_key(curr) {
            map.insert(curr.clone(), 0);
        }
        map.insert(curr.clone(), map.get(curr).unwrap() + 1);
        map
    })
}

// Given a map of frequencies, "normalize" them so that they are ordered 0-255.
// Note that this WILL work, as there are only 255 u8 values.
// The reason we're doing this is that storing a hashmap of u8-u8 is smaller.
// The fundamental objective of this program is to minimize storage size.
pub fn normalize(original: &HashMap::<u8, usize>) -> HashMap<u8, u8> {
   let mut storage_vec = original.iter().fold(Vec::new(), | mut vec, (byte, count) | {
       vec.push(FreqCount::new(byte.clone(), count.clone()));
       vec
   });

   storage_vec.sort();

   let mut retmap: HashMap::<u8, u8> = HashMap::new();
   for i in 0..storage_vec.len() {
       retmap.insert(storage_vec[i].byte, i as u8);
   }
   retmap
}

// A single node, capable of being filtered into a heap.
// Author: Will Morris.

// A node in this context is a data tuple.
#[derive(PartialEq, Eq, Debug)]
pub struct FreqCount {
    byte: u8,
    count: usize,
}

impl FreqCount {
    pub fn new(byte: u8, count: usize) -> FreqCount {
        Self { byte: byte, count: count }
    }
}

// Explicit ord implementation needed to ensure count considered first.
// default ord implementation would compare based on ordering of struct fields.
impl Ord for FreqCount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
            .then_with(|| self.byte.cmp(&other.byte))
    }
}

impl PartialOrd for FreqCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
