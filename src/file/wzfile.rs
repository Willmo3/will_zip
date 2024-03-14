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
use crate::file::bytestream::{ByteStream, long_to_bytes, min_byte_size, slice_to_long};
use crate::ordering::freqmap::{Freqmap, MAP_SIZE_FIELD_LEN, MAX_MAP_SIZE};

#[derive(Debug, Clone, PartialEq)]
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
    pub fn deconstruct(self) -> (HashMap<u8, u64>, BitSequence) {
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

        // Since there are only 256 bytes, maps have a tight upper bound on their size.
        let map_len = slice_to_long(&bytes[..MAP_SIZE_FIELD_LEN]) as usize;
        assert!(map_len <= MAX_MAP_SIZE);

        i += MAP_SIZE_FIELD_LEN;
        let map = Freqmap::from_stream(&bytes[i..i + map_len]);
        i += map_len;

        // However, there can be arbitrarily many characters in a file, so this length will
        // be encoded as a long.

        // In order to reduce the size of the bit len field, having a field for its length.
        let seq_len_len = bytes[i] as usize;
        i += 1;

        let seq_len = slice_to_long(&bytes[i..i + seq_len_len]) as usize;
        i += seq_len_len;
        let seq = BitSequence::from_stream(&bytes[i.. i + seq_len]);
        i += seq_len;

        assert_eq!(i, bytes.len());
        Wzfile::new(map.take(), seq)
    }

    fn to_stream(self) -> Vec<u8> {
        let mut retval = vec![];

        let mut map_bytes = self.map.to_stream();
        // Add length of frequency mapping
        retval.append(&mut long_to_bytes(map_bytes.len() as u64, MAP_SIZE_FIELD_LEN as u8));
        retval.append(&mut map_bytes);

        // Add length of sequence
        let mut seq_bytes = self.seq.to_stream();

        let size = seq_bytes.len() as u64;
        // Need to know the width of the seq size field for deserialization!
        let size_width = min_byte_size(size);
        retval.push(size_width);
        // Now, append that many bytes representing the size of the seq.
        retval.append(&mut long_to_bytes(size, size_width));
        // And finally, append the actual sequence
        retval.append(&mut seq_bytes);

        retval
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::encoding::bitsequence::BitSequence;
    use crate::file::bytestream::ByteStream;
    use crate::file::wzfile::Wzfile;

    #[test]
    fn test_no_len() {
        let empty_map = HashMap::new();
        let empty_seq = BitSequence::new();
        let expected = Wzfile::new(empty_map, empty_seq);

        let to = expected.clone().to_stream();
        let from = Wzfile::from_stream(&to);

        assert_eq!(expected, from);
    }

    #[test]
    fn test_real_deal() {
        let mut map: HashMap<u8, u64> = HashMap::new();
        for i in 0..20 {
            map.insert(i, i as u64 * i as u64);
        }

        let mut seq = BitSequence::new();
        for i in 0..33 {
            seq.append_bit(i % 2);
        }

        let expected = Wzfile::new(map, seq);

        let to = expected.clone().to_stream();
        let from = Wzfile::from_stream(&to);

        assert_eq!(expected, from);
    }
}