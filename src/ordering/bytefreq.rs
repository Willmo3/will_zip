use std::cmp::Ordering;

// An ordering of a byte to its frequency.
// This is useful for propagating into a heap later.
#[derive(PartialEq, Eq, Debug, Hash)]
pub struct ByteFreq {
    byte: u8,
    frequency: u64,
}

impl ByteFreq {
    pub fn new(byte: u8, frequency: u64) -> Self {
        Self { byte, frequency }
    }
    pub fn byte(&self) -> u8 {
        self.byte
    }
    pub fn freq(&self) -> u64 {
        self.frequency
    }
}

// Explicit ord implementation needed to ensure count considered first.
// default ord implementation would compare based on ordering of struct fields.
impl Ord for ByteFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
            .then_with(|| self.byte.cmp(&other.byte))
    }
}

impl PartialOrd for ByteFreq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}