use std::collections::HashMap;
use std::cmp::Ordering;

// Order the bytes in a stream based on how often they appear.
// Needed for compression
// Author: Will Morris

// Generate a frequency of all the bytes in a file.
pub fn gen_frequency(bytes: &[u8]) -> HashMap<u8, usize> {
    bytes.iter().fold(HashMap::<u8, usize>::new(), | mut map, curr | {
        if !map.contains_key(curr) {
            map.insert(*curr, 0);
        }
        map.insert(*curr, map.get(curr).unwrap() + 1);
        map
    })
}

// An ordering of a byte to its frequency.
// This is useful for propagating into a heap later.
#[derive(PartialEq, Eq, Debug, Hash)]
pub struct ByteFreq {
    byte: u8,
    frequency: usize,
}

impl ByteFreq {
    pub fn new(byte: u8, frequency: usize) -> Self {
        Self { byte, frequency }
    }
    pub fn byte(&self) -> u8 {
        self.byte
    }
    pub fn freq(&self) -> usize {
        self.frequency
    }
}

// Explicit ord implementation needed to ensure count considered first.
// default ord implementation would compare based on ordering of struct fields.
impl Ord for ByteFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
            .then_with(|| self.byte.cmp(&other.byte))
    }
}

impl PartialOrd for ByteFreq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}




