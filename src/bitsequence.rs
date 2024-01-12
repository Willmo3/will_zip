// A BitSequence encapsulates a string of bits and methods for interacting with them.
// Author: Will Morris
struct BitSequence {
    // Using u64 bc encoding a large file could exceed usize bits.
    num_bits: u64,
    bytes: Vec<u8>,
}

impl BitSequence {
    fn new() -> Self {
        Self {
            num_bits: 0,
            bytes: vec![],
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
}

