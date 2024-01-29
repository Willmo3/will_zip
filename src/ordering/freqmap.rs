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
    pub fn new() -> Self {
        Freqmap {
            data: HashMap::new()
        }
    }

    // Since a freqmap is just a wrapper for a hashmap, allow access to the reference.
    // This simplifies the external API.
    pub fn take(&mut self) -> &mut HashMap<u8, usize> {
        &mut self.data
    }
}


impl ByteStream for Freqmap {
    type Data = Freqmap;

    // Given a stream of bytes containing key-value pairs.
    // Convert that stream into a hashmap of those pairs.
    fn from_stream(bytes: &[u8]) -> Self::Data {
        const LONG_LEN: usize = size_of::<u64>();

        let mut deserialized = Freqmap::new();
        let map = deserialized.take();

        let mut i = 0;
        let bound = bytes.len() - 1 - LONG_LEN;

        while i < bound {
            let byte = bytes[i];
            i += 1;

            let mut buf = [0u8; LONG_LEN];
            buf.copy_from_slice(&bytes[i..i+LONG_LEN]);
            let val: usize = usize::from_le_bytes(buf);
            i+= LONG_LEN;

            map.insert(byte, val);
        }

        deserialized
    }

    fn to_stream(&self) -> Vec<u8> {
        todo!()
    }
}