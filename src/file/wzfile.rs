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

#[derive(Debug, Clone, PartialEq)]
pub struct Wzfile {
    map: Freqmap,
    seq: BitSequence
}

impl Wzfile {
    // Given a map and seq, Wzfile prepares compression.
    pub fn new(map: HashMap<u8, u64>, seq: BitSequence) -> Self {
        Wzfile { map: Freqmap::new(map, 8), seq }
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

        let map_len = slice_to_long(&bytes[..LONG_LEN]) as usize;
        i += LONG_LEN;
        let map = Freqmap::from_stream(&bytes[i..i + map_len]);
        i += map_len;

        let bit_len = slice_to_long(&bytes[i..i + LONG_LEN]) as usize;
        i += LONG_LEN;
        let bits = BitSequence::from_stream(&bytes[i.. i + bit_len]);
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

// Find the minimum number of bytes needed to represent all bytes
// Useful for serialization -- we don't want to end up encoding extra zeros in the hashmaps!
pub fn trim(values: &[u64]) -> u8 {
    values.iter().fold(1, | min_size: u8, datum | {
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
    use crate::encoding::bitsequence::BitSequence;
    use crate::file::bytestream::ByteStream;
    use crate::file::wzfile::{trim, Wzfile};

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

    #[test]
    fn test_trim() {
        let values = vec![512, 32, 4, 8, 0, 22, 43];
        assert_eq!(2, trim(&values))
    }
}