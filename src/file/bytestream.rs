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

// Given a slice of bytes, convert them into a u64.
pub(crate) fn slice_to_long(bytes: &[u8]) -> u64 {
    let mut buf = [0u8; LONG_LEN];
    buf[..bytes.len()].copy_from_slice(bytes);
    u64::from_le_bytes(buf)
}

// Given a long, convert it to a byte array of size size.
// NOTE: size must be >= minimum bytes to represent this data!
// Also, size must be at least one. Not representing 0 with zero bytes!
pub(crate) fn long_to_bytes(value: u64, size: u8) -> Vec<u8> {
    let min_size = min_byte_size(value);
    assert!(size > 0 && size >= min_size);
    // Requiring size be sent as u8 to establish upper bound on max size.
    let size = size as usize;

    let mut retval = vec![0u8; size];
    let data_bytes = value.to_le_bytes();
    retval[..size].copy_from_slice(&data_bytes[..size]);

    retval
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

    // Special case: all zeroes.
    // We need at least one byte to represent this!
    if leading_zeros == 8 {
        leading_zeros = 7
    }

    (LONG_LEN - leading_zeros) as u8
}

#[cfg(test)]
mod tests {
    use crate::file::bytestream::{long_to_bytes, min_byte_size, slice_to_long};

    #[test]
    fn test_slice_to_long() {
        let data = vec![1, 1];
        let value = slice_to_long(&data);
        assert_eq!(257, value)
    }

    #[test]
    fn test_long_to_slice() {
        assert_eq!(vec![1, 1], long_to_bytes(257, 2));
        assert_eq!(vec![0], long_to_bytes(0, 1));
    }

    #[test]
    fn test_min_byte_size() {
        assert_eq!(8, min_byte_size(18446744073709551615));
        assert_eq!(1, min_byte_size(1));
        assert_eq!(1, min_byte_size(0));
    }
}