use std::env;
use std::process;
use std::fs;
use crate::ordering::freq::gen_ordering;
use crate::tree::node::Node;

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
    // Abstract ordering of byte to precedence.
    // Used by freq to order by frequency.
    pub mod byteordering;
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
}

fn main() {
    // For now, only checking that there is a second arg.
    // Not complaining if they pass too many.
    let filename: String = match env::args().nth(1) {
        Some(name) => name,
        None => {
            println!("File name not specified! Usage: wz [filename]");
            process::exit(1)
        },
    };

    let bytes: Vec<u8> = match fs::read(&filename) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("File not found: {}", &filename);
            process::exit(1)
        },
    };

    let ordering = gen_ordering(&bytes);
    let heap = Node::huffman(&ordering);

    // Create an empty file, do not do any additional work.
    // This allows future encoding to rely on no "nones" being present.
    if heap.is_none() {
        return;
    }

    let heap = heap.unwrap();
    println!("{}", heap);
}
