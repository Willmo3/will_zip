// A BitSequence encapsulates a string of bits and methods for interacting with them.
// Author: Will Morris
// Big credit to Dr. Nathan Sprague for making a java version of this.
struct BitSequence {
    // NOTE: in most cases, u64 will be equal to usize, so indexing with u64 will work.
    // The only time this wouldn't work is:
    // 1. you're on a 32 bit system
    // 2. you attempt to access an index larger than the u32 size limit
    // (i.e. when compressing a very large file)
    // In this case, the overflow will cause a panic, avoiding undefined behavior.
    num_bits: u64,
    bytes: Vec<u8>,
}

impl BitSequence {
    // Create a new, empty BitSequence.
    fn new() -> Self {
        Self {
            num_bits: 0,
            bytes: vec![],
        }
    }

    // Append all bits from iterator to the end of this BitSequence.
    fn append_all(&mut self, iter: &mut BitIterator) {
        iter.for_each(| bit | self.append_bit(bit));
    }

    // Append a single bit to the end of the sequence.
    fn append_bit(&mut self, bit: u8) {
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

    // Get the bit at index usize.
    fn get_bit(&self, index: u64) -> Option<u8> {
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

    // Get immutable iterator over this sequence.
    fn iter(&self) -> BitIterator {
        BitIterator { sequence: &self, index: 0 }
    }
}

// Many operations will operate over entire sequences.
// For this reason, a BitIterator is provided.
struct BitIterator<'a> {
    sequence: &'a BitSequence,
    index: u64,
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let retval = self.sequence.get_bit(self.index);
        self.index += 1;
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
    fn test_iter() {
        let mut seq = BitSequence::new();
        for i in 0..64 {
            seq.append_bit(i % 2);
        }

        let mut iter = seq.iter();
        for i in 0..64 {
            assert_eq!(iter.next().unwrap(), i % 2);
        }
    }

    #[test]
    fn test_append_all() {
        let mut seq1 = BitSequence::new();
        for i in 0..64 {
            seq1.append_bit(i % 2);
        }

        let mut seq2 = BitSequence::new();
        for i in 0..64 {
            seq2.append_bit((i + 1) % 2);
        }

        seq1.append_all(&mut seq2.iter());
        assert_eq!(0, seq1.get_bit(127).unwrap());
    }
}

