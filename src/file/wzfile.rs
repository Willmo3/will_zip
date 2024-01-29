// Represents an encoded wzfile.
// Author: Will Morris

/*
  CONTENTS:
  -- length of frequency map
  -- actual frequency map
  -- num bytes
  -- bytestream.
 */

pub struct wzfile {
    // Ultimately, the wzfile is a stream of bytes.
    // Its contents are:
    // HashMap::(byte, freq as u64)
    // bitsequence of encoded file.
}

impl wzfile {
    // We need:
    // -- A way to get this from bytes.
    //    Steps:
    //    1. Decode map
    //    2. Decode BitSequence.
    // Simple: each must be able to decode themselves.
}