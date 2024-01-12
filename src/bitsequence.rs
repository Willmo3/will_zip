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

    // Append a single bit to the end of the sequence.
    fn append_bit(&mut self, bit: u8) {
        assert!(bit == 0 || bit == 1);
        self.num_bits += 1;

        let byte_index = self.num_bits / 8;
        if byte_index >= self.bytes.len() as u64 {
            self.bytes.push(0);
        }

        let bit_index = self.num_bits % 8;
        let mask = 1 << bit_index;
        self.bytes[byte_index as usize] |= mask;
    }

    // Get the bit at index usize.
    fn get_bit(&self, index: u64) -> u8 {
        assert!(index < self.num_bits);
        let byte_index = index / 8;
        let bit_index = index % 8;
        let mask = 1 << bit_index;
        match mask & self.bytes.get(byte_index as usize).unwrap() {
            0 => { 0 },
            _ => { 1 },
        }
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
        assert_eq!(0, seq.get_bit(0));
        assert_eq!(1, seq.get_bit(1));

        for _ in 0..32 {
            seq.append_bit(1);
        }
        assert_eq!(1, seq.get_bit(32));
        assert_eq!(1, seq.get_bit(1));
        assert_eq!(0, seq.get_bit(0));
    }
}

