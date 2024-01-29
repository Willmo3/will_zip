use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::file::bytestream::{ByteStream, LONG_LEN, slice_to_long};

// A BitSequence encapsulates a string of bits and methods for interacting with them.
// Author: Will Morris
// Big credit to Dr. Nathan Sprague for making a java version of this.
type Bit = u8;

#[derive(Clone, PartialEq)]
pub(crate) struct BitSequence {
    // NOTE: in most cases, u64 will be equal to usize, so indexing with u64 will work.
    // The only time this wouldn't work is:
    // 1. you're on a 32-bit system
    // 2. you attempt to access an index larger than the u32 size limit
    // (i.e. when compressing a very large file)
    // In this case, the overflow will cause a panic, avoiding undefined behavior.
    num_bits: u64,
    bytes: Vec<Bit>,
}


// ****** CONSTRUCTORS ****** //
impl BitSequence {
    // Create a new, empty BitSequence.
    pub(crate) fn new() -> Self {
        Self {
            num_bits: 0,
            bytes: vec![],
        }
    }

    // Create a BitSequence from a string of bits.
    pub(crate) fn from_bits(bits: &[Bit]) -> Self {
        let mut seq = Self::new();
        seq.append_bits(bits);
        seq
    }

    // Create a BitSequence from a vector and length in bits.
    pub(crate) fn from(num_bits: u64, bytes: &[u8]) -> Self {
        Self { num_bits, bytes: bytes.to_vec() }
    }

    // Translate a collection of bytes into a large bitsequence.
    pub(crate) fn translate(bytes: &[u8], encoding: &HashMap<u8, BitSequence>) -> Self {
        let mut retval = BitSequence::new();
        for byte in bytes {
            retval.append_seq(encoding.get(byte).unwrap());
        }
        retval
    }
}


// ****** MUTATORS ****** ///
impl BitSequence {
    // Append a single bit to the end of the sequence.
    pub(crate) fn append_bit(&mut self, bit: Bit) {
        assert!(bit == 0 || bit == 1);

        let byte_index = self.num_bits / 8;
        if byte_index >= self.bytes.len() as u64 {
            self.bytes.push(0);
        }

        if bit != 0 {
            let bit_index = self.num_bits % 8;
            let mask = 1 << bit_index;
            self.bytes[byte_index as usize] |= mask;
        }

        self.num_bits += 1;
    }

    // Append all bits from bit slice to self.
    // Useful for adding all bits while maintaining ownership.
    pub(crate) fn append_bits(&mut self, bits: &[Bit]) {
        bits.iter().for_each(|bit| self.append_bit(*bit));
    }

    // Assimilate a BitSequence into this sequence.
    // Useful for removing temporary BitSequences from the equation
    // if you want to keep your BitSequence, use append_bits
    fn append_seq(&mut self, seq: &BitSequence) {
        self.append_bits(&seq.get_bits());
    }
}


// ****** ACCESSORS ****** //

impl BitSequence {
    // Get the bit at index usize.
    fn get_bit(&self, index: u64) -> Option<Bit> {
        if index >= self.num_bits {
            return None;
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        let mask = 1 << bit_index;
        match mask & self.bytes.get(byte_index as usize).unwrap() {
            0 => { Some(0) },
            _ => { Some(1) },
        }
    }

    // Get all bits in bit sequence.
    fn get_bits(&self) -> Vec<u8> {
        let mut bits: Vec<Bit> = vec![];
        for i in 0..self.num_bits {
            bits.push(self.get_bit(i).unwrap());
        }
        bits
    }

    // Length attribute particularly useful when testing.
    pub(crate) fn length(&self) -> u64 {
        self.num_bits
    }
}

impl Debug for BitSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for bit in self.get_bits() {
            let result = f.write_fmt(format_args!("{}", bit));
            if result.is_err() {
                return result
            }
        }
        Ok(())
    }
}

impl ByteStream for BitSequence {
    type Data = BitSequence;

    fn from_stream(bytes: &[u8]) -> Self::Data {
        let num_bits = slice_to_long(&bytes[..LONG_LEN]);
        let data = &bytes[LONG_LEN..];
        BitSequence::from(num_bits, data)
    }

    fn to_stream(mut self) -> Vec<u8> {
        let mut retval = vec![];
        retval.append(&mut Vec::from(self.num_bits.to_le_bytes()));
        retval.append(&mut self.bytes);
        retval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct() {
        BitSequence::new();
    }

    #[test]
    fn test_append_get() {
        let mut seq = BitSequence::new();

        seq.append_bit(0);
        seq.append_bit(1);
        seq.append_bit(0);
        assert_eq!(0, seq.get_bit(0).unwrap());
        assert_eq!(1, seq.get_bit(1).unwrap());

        for _ in 0..32 {
            seq.append_bit(1);
        }
        assert_eq!(1, seq.get_bit(32).unwrap());
        assert_eq!(1, seq.get_bit(1).unwrap());
        assert_eq!(0, seq.get_bit(0).unwrap());
    }

    #[test]
    fn test_append_seq() {
        let mut seq1 = BitSequence::new();
        for i in 0..64 {
            seq1.append_bit(i % 2);
        }

        let mut seq2 = BitSequence::new();
        for i in 0..64 {
            seq2.append_bit((i + 1) % 2);
        }

        seq1.append_seq(&seq2);
        assert_eq!(0, seq1.get_bit(127).unwrap());
    }
}

#[cfg(test)]
mod serialize_tests {
    use crate::encoding::bitsequence::BitSequence;
    use crate::file::bytestream::ByteStream;

    #[test]
    fn test_empty_bitseq() {
        let seq = BitSequence::new();
        let from = seq.to_stream();
        let to = BitSequence::from_stream(&from);
        assert_eq!(0, to.num_bits);
    }

    #[test]
    fn test_real_bitseq() {
        let mut seq = BitSequence::new();
        for i in 0..10 {
            seq.append_bit(0);
        }
        seq.append_bit(1);

        let bytes = seq.clone().to_stream();
        let from = BitSequence::from_stream(&bytes);

        assert_eq!(seq, from);
    }
}

