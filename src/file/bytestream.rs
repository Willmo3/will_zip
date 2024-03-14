// Author: Will Morris
// This represents any data that can be constructed from a stream of bytes.
// This will be used for efficient serialization.

use std::mem::size_of;

pub trait ByteStream {
    type Data;

    // Given a slice of bytes of a fixed size
    // an object of this type can be constructed.
    fn from_stream(bytes: &[u8]) -> Self::Data;

    // This function converts self to a byte vector, taking ownership.
    // Typically, converting into a stream is the last step before file serialization.
    // However, if you need self back, from_stream will work on a proper implementation.
    fn to_stream(self) -> Vec<u8>;
}

pub(crate) const LONG_LEN: usize = size_of::<u64>();

// TODO: add a long to slice?
// Need to have wrappers for the variable length fields.

// Given a slice of bytes, convert them into a u64.
pub(crate) fn slice_to_long(bytes: &[u8]) -> u64 {
    let mut buf = [0u8; LONG_LEN];
    for i in 0..bytes.len() {
        buf[i] = bytes[i]
    }
    u64::from_le_bytes(buf)
}

// Get the minimum number of bytes needed to represent a 64-bit value.
pub(crate) fn min_byte_size(value: u64) -> u8 {
    let data_bytes = value.to_be_bytes();

    // How many leading zeros do we have?
    // These could just as easily be ignored.
    let mut leading_zeros = 0;
    for byte in data_bytes {
        if byte != 0 {
            break
        }
        leading_zeros += 1
    }

    return (LONG_LEN - leading_zeros) as u8
}

#[cfg(test)]
mod tests {
    use crate::file::bytestream::slice_to_long;

    #[test]
    fn test_slice_to_long() {
        let data = vec![1, 1];
        let value = slice_to_long(&data);
        assert_eq!(257, value)
    }
}