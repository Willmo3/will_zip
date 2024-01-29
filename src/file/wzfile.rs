// Represents an encoded wzfile.
// Author: Will Morris

/*
  CONTENTS:
  -- length of frequency map
  -- actual frequency map
  -- num bytes
  -- bytestream.
 */

use std::collections::HashMap;
use crate::encoding::bitsequence::BitSequence;
use crate::file::bytestream::{ByteStream, LONG_LEN, slice_to_long};
use crate::ordering::freqmap::Freqmap;

pub struct Wzfile {
    map: Freqmap,
    seq: BitSequence
}

impl Wzfile {
    // Given a map and seq, Wzfile prepares compression.
    pub fn new(map: HashMap<u8, u64>, seq: BitSequence) -> Self {
        Wzfile { map: Freqmap::new(map), seq }
    }

    // Once a wzfile has been deserialized, deconstruct it for access to its fields.
    pub fn deconstruct(self) -> (HashMap::<u8, u64>, BitSequence) {
        let map = self.map.take();
        let seq = self.seq;
        (map, seq)
    }
}

impl ByteStream for Wzfile {
    type Data = Wzfile;

    // Given a byte array, deconstruct it into its component byte fields.
    // Which will then deserialize themselves.
    fn from_stream(bytes: &[u8]) -> Self::Data {
        let mut i = 0;

        let map_len: usize = slice_to_long(&bytes[..LONG_LEN]) as usize;
        i += LONG_LEN;
        let map = Freqmap::from_stream(&bytes[i..map_len as usize]);
        i += map_len;

        let bit_len = slice_to_long(&bytes[i..LONG_LEN]) as usize;
        i += LONG_LEN;
        let bits = BitSequence::from_stream(&bytes[i..]);
        i += bit_len;

        assert_eq!(i, bytes.len());

        Wzfile::new(map.take(), bits)

    }

    fn to_stream(self) -> Vec<u8> {
        let mut retval = vec![];

        let mut map_bytes = self.map.to_stream();
        // Add length of frequency mapping
        retval.append(&mut Vec::from(map_bytes.len().to_le_bytes()));
        retval.append(&mut map_bytes);

        // Add length of sequence
        let mut seq_bytes = self.seq.to_stream();
        retval.append(&mut Vec::from(seq_bytes.len().to_le_bytes()));
        retval.append(&mut seq_bytes);

        retval
    }
}