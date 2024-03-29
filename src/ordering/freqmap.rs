// FreqMap contains a map from u8->usize.
// That can be serialized and deserialized.
// This will be useful for encoding as a file.
// Author: Will Morris

use std::collections::HashMap;
use crate::file::bytestream::{ByteStream, LONG_LEN, long_to_bytes, min_byte_size, slice_to_long};

// What's the maximum number of bytes needed to represent the contents of a freqmap in memory?
// 9 bytes per 256 entries, plus one byte for the per-entry size field.
pub(crate) const MAX_MAP_SIZE: usize = (LONG_LEN + 1) * 256 + 1;
// How many bytes will it take to represent the size field for our map?
// In this case, MAX_MAP_SIZE can be represented as u16 (with lots of spare space!).
pub(crate) const MAP_SIZE_FIELD_LEN: usize = 2;

#[derive(Debug, Clone, PartialEq)]
pub struct Freqmap {
    data: HashMap<u8, u64>
}

impl Freqmap {
    pub fn new(map: HashMap<u8, u64>) -> Self {
        Freqmap { data: map }
    }

    // FreqMap is really just a wrapper for serialization.
    // Therefore, it is acceptable to take ownership when you need the map.
    pub fn take(self) -> HashMap<u8, u64> {
        self.data
    }
}

// Primary purpose of freqmap: enable serialization
impl ByteStream for Freqmap {
    type Data = Freqmap;

    // Given a stream of bytes containing key-value pairs.
    // Convert that stream into a hashmap of those pairs.
    fn from_stream(bytes: &[u8]) -> Self::Data {
        let mut map: HashMap<u8, u64> = HashMap::new();
        let size = bytes[0] as usize;

        // premature exit: too small!
        // Extra byte for the first entry.
        if bytes.len() <= size + 1 {
            return Freqmap::new(map);
        }

        let bound= bytes.len() - size;
        // Start adding key-value pairs after the size field.
        let mut i = 1;

        while i < bound {
            let byte = bytes[i];
            i += 1;
            let val = slice_to_long(&bytes[i..i+size]);
            i+= size;
            map.insert(byte, val);
        }

        Freqmap::new(map)
    }

    // Convert one of these bad boys into a byte stream.
    fn to_stream(self) -> Vec<u8> {
        let mut retval = Vec::new();
        let data = self.take();

        let size = trim_map(&data);
        retval.push(size);

        for (byte, value) in data {
            retval.push(byte);
            retval.append(&mut long_to_bytes(value, size));
        }
        retval
    }
}

// Find the minimum number of bytes needed to represent values in map
// Useful for serialization -- we don't want to end up encoding extra zeros in the hashmaps!
fn trim_map(map: &HashMap<u8, u64>) -> u8 {
    map.values().fold(1, |min_size: u8, datum | {
        let size = min_byte_size(*datum);
        if size > min_size {
            size
        } else {
            min_size
        }
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::file::bytestream::ByteStream;
    use crate::ordering::freqmap::{Freqmap, trim_map};

    #[test]
    fn test_empty_to() {
        // An empty map would have size 1
        let bytes = vec![1];
        let to = Freqmap::from_stream(&bytes);
        let from = to.to_stream();
        assert_eq!(bytes, from);
    }

    #[test]
    fn test_to_from() {
        let mut map = HashMap::new();
        map.insert(0, 52);
        map.insert(4, 14);
        map.insert(1, 22);

        let from = Freqmap::new(map.clone()).to_stream();
        let to = Freqmap::from_stream(&from);

        let to_map = to.take();
        assert_eq!(map, to_map);
    }


    #[test]
    fn test_trim_map() {
        let mut map = HashMap::new();
        map.insert(1, 12);
        map.insert(2, 512);
        assert_eq!(2, trim_map(&map));

        map.insert(3, 18446744073709551615);
        assert_eq!(8, trim_map(&map));
    }
}