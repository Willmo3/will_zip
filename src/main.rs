use std::env;
use std::process;
use std::fs;
use std::fs::File;
use std::io::Write;
use getopt::Opt;
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
    // All arguments.
    let mut input_file: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut zip = false;
    let mut unzip = true;

    if !parse_args(&mut input_file, &mut output_file, &mut zip, &mut unzip) {
        println!("Terminating.");
        process::exit(1)
    };

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

// Parses args.
// Grabs the input and output filenames, if applicable.
// Grabs whether the input file is being zipped or unzipped.
// Return whether the program should continue.
fn parse_args(input_filename: &mut Option<String>,
              output_filename: &mut Option<String>,
              zip: &mut bool,
              unzip: &mut bool) -> bool {

    let args: Vec<String> = env::args().collect();
    let mut ops = getopt::Parser::new(&args, "zxi:o:");
    loop {
        match ops.next().transpose().unwrap() {
            None => break,
            Some(opt) => match opt {
                Opt('z', None)             => { *zip = true; }
                Opt('x', None)             => { *unzip = true; }
                Opt('i', Some(str)) => { *input_filename = Some(str.clone()); }
                Opt('o', Some(str)) => { *output_filename = Some(str.clone()); }
                Opt('u', None) => {
                    usage();
                    return false;
                }
                Opt(value, _) => {
                    println!("Unrecognized arg: {}", value);
                    usage();
                    return false;
                }
            }
        }
    }
    true
}

fn usage() {
    println!("Usage: wz");
    println!("-u (usage)");
    println!("-i (input file)");
    println!("-o (output file)");
    println!("-z (compress input file, mutually exclusive with -x)");
    println!("-x (extract input file, mutually exclusive with -z)")
}
