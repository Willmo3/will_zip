use std::env;
use std::process;
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::encoding::bitsequence::BitSequence;
use crate::file::bytestream::ByteStream;
use crate::file::wzfile::Wzfile;
use crate::ordering::freq::gen_frequency;
use crate::tree::node::{huffman, Node};

// Given a file F, this program converts F into a HuffmanEncoding and saves a copy of it
// Or given an already-encoded file F', this program converts it into a decoded file F.
// Author: Will Morris

mod tree {
    pub mod node;
}

// The core of the program revolves around ordering bytes by their precedence.
mod ordering {
    // Generates an ordering of bytes-frequency of appearance.
    pub mod freq;
    pub(crate) mod bytefreq;
    pub(crate) mod freqmap;
}

// Encodings are used when serializing the file to save space.
mod encoding {
    // Represents a list of bits, compressed using bitwise ops into a vec<u8>
    pub mod bitsequence;
}

// Relevant to the actual act of saving the file.
mod file {
    // Anything which can be represented as a stream of bytes uses this trait.
    // This allows for easier deserialization... given a byte array, an object will come out!
    pub mod bytestream;
    pub mod wzfile;
}

fn main() {
    // For now, only checking that there is a second arg.
    // Not complaining if they pass too many.
    let filename: String = env::args().nth(1).unwrap_or_else(|| {
        println!("File name not specified! Usage: wz [filename]");
        process::exit(1)
    });

    let bytes: Vec<u8> = fs::read(&filename).unwrap_or_else(|_| {
        println!("File not found: {}", &filename);
        process::exit(1)
    });

    let ordering = gen_frequency(&bytes);
    let heap = huffman(&ordering);

    // Create an empty file, do not do any additional work.
    // This allows future encoding to rely on no "nones" being present.
    if heap.is_none() {
        return;
    }

    let heap = heap.unwrap();
    let encoding = heap.gen_encoding();
    let seq = BitSequence::translate(&bytes, &encoding);
    let bytes = Wzfile::new(ordering, seq).to_stream();

    let mut output = File::create("test.wz").unwrap();
    output.write_all(&bytes).unwrap();
}
