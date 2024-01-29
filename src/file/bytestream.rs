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
    assert_eq!(bytes.len(), LONG_LEN);

    let mut buf = [0u8; LONG_LEN];
    buf.copy_from_slice(&bytes[0..]);
    u64::from_le_bytes(buf)
}