// FreqMap contains a map from u8->usize.
// That can be serialized and deserialized.
// This will be useful for encoding as a file.
// Author: Will Morris

use std::collections::HashMap;
use crate::file::bytestream::{ByteStream, LONG_LEN, slice_to_long};

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
        let size = bytes[0];

        // premature exit: too small!
        // Extra byte for the first entry.
        if bytes.len() <= (size as usize) + 1 {
            return Freqmap::new(map);
        }

        let bound= bytes.len() - (size as usize);
        // Start adding key-value pairs after the size field.
        let mut i = 1;

        while i < bound {
            let byte = bytes[i];
            i += 1;
            let val = slice_to_long(&bytes[i..i+LONG_LEN]);
            i+= LONG_LEN;
            map.insert(byte, val);
        }

        Freqmap::new(map)
    }

    // Convert one of these bad boys into a byte stream.
    fn to_stream(self) -> Vec<u8> {
        let mut retval = Vec::new();
        let data = self.take();

        let size = trim(&data);
        retval.push(size);

        for (byte, value) in data {
            retval.push(byte);
            retval.append(&mut Vec::from(value.to_le_bytes()));
        }
        retval
    }
}

// Find the minimum number of bytes needed to represent all bytes
// Useful for serialization -- we don't want to end up encoding extra zeros in the hashmaps!
fn trim(map: &HashMap<u8, u64>) -> u8 {
    map.values().fold(1, |min_size: u8, datum | {
        let data_bytes = datum.to_be_bytes();

        // How many leading zeros do we have?
        // These could just as easily be ignored.
        let mut leading_zeros = 0;
        for byte in data_bytes {
            if byte != 0 {
                break
            }
            leading_zeros += 1
        }

        let size = (LONG_LEN as u8) - leading_zeros;

        // If it took more space to allocate this element than we currently allocated
        // Then our minimum size needs to be larger!
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
    use crate::ordering::freqmap::{Freqmap, trim};

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
    fn test_trim() {
        let mut map = HashMap::new();
        map.insert(1, 12);
        map.insert(2, 512);
        assert_eq!(2, trim(&map));

        map.insert(3, 18446744073709551615);
        assert_eq!(8, trim(&map));
    }
}