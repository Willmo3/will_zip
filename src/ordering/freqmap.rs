// FreqMap contains a map from u8->usize.
// That can be serialized and deserialized.
// This will be useful for encoding as a file.
// Author: Will Morris

use std::collections::HashMap;
use crate::file::bytestream::{ByteStream, LONG_LEN, slice_to_long};

#[derive(Debug, Clone, PartialEq)]
pub struct Freqmap {
    // although data contained in u64, must maintain minimum size for serialization.
    data_size: u8,
    data: HashMap<u8, u64>
}

impl Freqmap {
    pub fn new(map: HashMap<u8, u64>, data_size: u8) -> Self {
        Freqmap { data_size, data: map }
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
            return Freqmap::new(map, size);
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

        Freqmap::new(map, size)
    }

    // Convert one of these bad boys into a byte stream.
    fn to_stream(self) -> Vec<u8> {
        let mut retval = Vec::new();
        retval.push(self.data_size);
        let data = self.take();
        for (byte, value) in data {
            retval.push(byte);
            retval.append(&mut Vec::from(value.to_le_bytes()));
        }
        retval
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::file::bytestream::ByteStream;
    use crate::ordering::freqmap::Freqmap;

    #[test]
    fn test_empty_to() {
        let bytes = vec![8];
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

        let from = Freqmap::new(map.clone(), 8).to_stream();
        let to = Freqmap::from_stream(&from);

        let to_map = to.take();
        assert_eq!(map, to_map);
    }
}