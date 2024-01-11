use std::cmp::Ordering;
// A single node, capable of being filtered into a heap.
// Author: Will Morris.

// A node in this context is a data tuple.
#[derive(PartialEq, Eq, Debug)]
pub struct ByteOrdering {
    byte: u8,
    precedence: usize,
}

impl ByteOrdering {
    pub fn new(byte: u8, precedence: usize) -> ByteOrdering {
        Self { byte, precedence }
    }

    pub fn byte(&self) -> u8 {
        self.byte
    }
    pub fn precedence(&self) -> usize {
        self.precedence
    }
}

// Explicit ord implementation needed to ensure count considered first.
// default ord implementation would compare based on ordering of struct fields.
impl Ord for ByteOrdering {
    fn cmp(&self, other: &Self) -> Ordering {
        self.precedence.cmp(&other.precedence)
            .then_with(|| self.byte.cmp(&other.byte))
    }
}

impl PartialOrd for ByteOrdering {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}