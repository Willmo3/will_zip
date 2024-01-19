// Author: Will Morris
// This represents any data that can be constructed from a stream of bytes.
// This will be used for efficient serialization.

pub trait ByteStream {
    type Data;

    // Given a slice of bytes of a fixed size
    // an object of this type can be constructed.
    fn from_stream(bytes: &[u8]) -> Self::Data;

    // Any implementing type can be converted into a vector of bytes.
    fn to_stream(&self) -> Vec<u8>;
}