// Author: Will Morris
// This represents any data that can be constructed from a stream of bytes.
// This will be used for efficient serialization.

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