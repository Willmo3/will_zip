use std::env;
use std::process;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::{exit, Output};
use getopts::Options;
use crate::encoding::bitsequence::BitSequence;
use crate::file::bytestream::ByteStream;
use crate::file::wzfile::Wzfile;
use crate::ordering::freq::gen_frequency;
use crate::tree::node::{huffman, Node};

// Given a file F, this program converts F into a HuffmanEncoding and saves a copy of it
// Or given an already-encoded file F', this program converts it into a decoded file F.
// Author: Will Morris

mod tree {
    pub(crate) mod node;
}

// The core of the program revolves around ordering bytes by their precedence.
mod ordering {
    // Generates an ordering of bytes-frequency of appearance.
    pub(crate) mod freq;
    pub(crate) mod bytefreq;
    pub(crate) mod freqmap;
}

// Encodings are used when serializing the file to save space.
mod encoding {
    // Represents a list of bits, compressed using bitwise ops into a vec<u8>
    pub(crate) mod bitsequence;
}

// Relevant to the actual act of saving the file.
mod file {
    // Anything which can be represented as a stream of bytes uses this trait.
    // This allows for easier deserialization... given a byte array, an object will come out!
    pub(crate) mod bytestream;
    pub(crate) mod wzfile;
}

fn main() {
    let mut input_file = String::new();
    let mut output_file= String::new();
    let mut zip = false;
    // Unzip isn't strictly necessary, but I'm keeping it around for potential future use.
    let mut unzip = false;

    if let Some(exit_code) =
        parse_args(&mut input_file, &mut output_file, &mut zip, &mut unzip) {
        println!("Terminating.");
        exit(exit_code)
    };

    // Now, prepare input and output data for compression.
    let bytes: Vec<u8> = match fs::read(&input_file) {
        Ok(val) => { val }
        Err(_) => {
            println!("File not found: {}", &input_file);
            exit(1)
        }
    };

    // We've validated that zip or unzip must be true.
    // So no need to check unzip here -- if not zip, then go!
    let to_write = match zip {
        true => { compress(&bytes) }
        false => { decompress(&bytes) }
    };

    let mut output_file = File::create(output_file).unwrap();
    output_file.write_all(&to_write).unwrap();
    exit(0)
}


// ****** COMPRESSOR ****** //

// Returns exit status of program
fn compress(bytes: &[u8]) -> Vec<u8>{
    let ordering = gen_frequency(&bytes);
    let heap = huffman(&ordering);

    // Create an empty file, do not do any additional work.
    // This allows future encoding to rely on no "nones" being present.
    if heap.is_none() {
        return vec![]
    }

    let heap = heap.unwrap();
    let encoding = heap.gen_encoding();
    let seq = BitSequence::translate(&bytes, &encoding);
    let bytes = Wzfile::new(ordering, seq).to_stream();
    bytes
}


// ****** DECOMPRESSOR ****** //

// Returns exit status of program
fn decompress(bytes: &[u8]) -> Vec<u8> {
    let (ordering, seq) = Wzfile::from_stream(bytes).deconstruct();
    let heap = huffman(&ordering);

    if heap.is_none() {
        return vec![]
    }

    let heap = heap.unwrap();
    // Need to gen decoding.
    let decoding = heap.gen_decoding();
    // Now, need to turn each bit in bitsequence into a regular byte in output file.

    let mut bytes = vec![];
    let mut current_seq = BitSequence::new();

    for i in 0..seq.length() {
        let current = seq.get_bit(i).unwrap();
        current_seq.append_bit(current);
        if let Some(byte) = decoding.get(&current_seq) {
            bytes.push(*byte);
            // Start searching from the next bit again.
            current_seq = BitSequence::new();
        }
    }

    bytes
}


// ****** ARGUMENT CHECKERS ****** //

// Parses args.
// Grabs the input and output filenames, if applicable.
// Grabs whether the input file is being zipped or unzipped.
// Validates that the combination is correct.
// Return either the exit code the program should give, or none.
fn parse_args(input_filename: &mut String,
              output_filename: &mut String,
              zip: &mut bool,
              unzip: &mut bool) -> Option<i32> {

    let args: Vec<String> = env::args().collect();
    // length one if no user args specified.
    if args.len() == 1 {
        usage();
        return Some(0)
    }

    // Credit to getopts documentation for this.
    // https://docs.rs/getopts/latest/getopts/
    let mut opts = Options::new();
    opts.optopt("o", "output", "output file name", "out.wz");
    opts.optopt("i", "input", "input file name", "in.txt");
    opts.optflag("u", "usage", "print this usage menu");
    opts.optflag("z", "zip", "compress input file");
    opts.optflag("x", "extract", "extract input file");

    let matches = match opts.parse(&args[1..]) {
        Ok( m) => { m }
        Err( f) => {
            println!("{}", f);
            usage();
            return Some(1)
        }
    };

    if matches.opt_present("u") {
        usage();
        return Some(0)
    }

    if matches.opt_present("x") {
        *unzip = true
    }
    if matches.opt_present("z") {
        *zip = true
    }
    if *zip == *unzip {
        println!("Must either zip or unzip a file!");
        usage();
        return Some(1)
    }

    *input_filename = match matches.opt_str("i") {
        None => {
            println!("No input file specified!");
            return Some(1);
        }
        Some(val) => { val }
    };
    *output_filename = match matches.opt_str("o") {
        None => {
            println!("No output file specified!");
            return Some(1);
        }
        Some(val) => { val }
    };

    // If we get all the way here, no exit code. Keep the program going!
    None
}

fn usage() {
    println!("Usage: wz");
    println!("-u (usage)");
    println!("-i (input file)");
    println!("-o (output file)");
    println!("-z (compress input file, mutually exclusive with -x)");
    println!("-x (extract input file, mutually exclusive with -z)")
}
