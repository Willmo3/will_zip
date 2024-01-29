// FreqMap contains a map from u8->usize.
// That can be serialized and deserialized.
// This will be useful for encoding as a file.
// Author: Will Morris

use std::collections::HashMap;
use std::mem::size_of;
use crate::file::bytestream::ByteStream;

pub struct Freqmap {
    data: HashMap<u8, usize>
}

impl Freqmap {
    pub fn new(map: HashMap<u8, usize>) -> Self {
        Freqmap {
            data: map
        }
    }

    // Since a freqmap is just a wrapper for a hashmap, allow access to the reference.
    // This can then be cloned for its relevant uses later.
    pub fn take(&self) -> &HashMap<u8, usize> {
        &self.data
    }
}


impl ByteStream for Freqmap {
    type Data = Freqmap;

    // Given a stream of bytes containing key-value pairs.
    // Convert that stream into a hashmap of those pairs.
    fn from_stream(bytes: &[u8]) -> Self::Data {
        const LONG_LEN: usize = size_of::<u64>();

        let mut map: HashMap<u8, usize> = HashMap::new();
        // premature exit: too small!
        if bytes.len() <= LONG_LEN + 1 {
            return Freqmap::new(map);
        }

        let bound= bytes.len() - 1 - LONG_LEN;
        let mut i = 0;
        
        while i < bound {
            let byte = bytes[i];
            i += 1;

            let mut buf = [0u8; LONG_LEN];
            buf.copy_from_slice(&bytes[i..i+LONG_LEN]);
            let val: usize = usize::from_le_bytes(buf);
            i+= LONG_LEN;

            map.insert(byte, val);
        }

        Freqmap::new(map)
    }

    // Convert one of these bad boys to a byte stream.
    fn to_stream(&self) -> Vec<u8> {
        let mut retval = Vec::new();
        let data = self.take().clone();
        for (byte, value) in data {
            retval.push(byte);
            retval.append(&mut Vec::from(value.to_le_bytes()));
        }
        retval
    }
}

#[cfg(test)]
mod tests {
    use crate::file::bytestream::ByteStream;
    use crate::ordering::freqmap::Freqmap;

    #[test]
    fn test_empty_to() {
        let bytes = vec![];
        let to = Freqmap::from_stream(&bytes);
        let from = Freqmap::to_stream(&to);
        assert_eq!(bytes, from);
    }
}