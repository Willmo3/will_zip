use std::env;
use std::process;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::{exit, Output};
use getopt::{Error, Opt};
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
    let mut input_file: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut zip = false;
    let mut unzip = false;

    if let Some(exit_code) =
        parse_args(&mut input_file, &mut output_file, &mut zip, &mut unzip) {

        println!("Terminating.");
        exit(exit_code)
    };

    if let Some(exit_code) =
        validate_args(&input_file, &output_file, &zip, &unzip) {

        println!("Terminating.");
        exit(exit_code)
    };

    let input_file: String = input_file.unwrap();
    let output_file: String = output_file.unwrap();
    // We've validated that zip or unzip must be true.
    // So no need to check unzip here -- if not zip, then go!
    if zip {
        exit(compress(&input_file, &output_file))
    } else {
        exit(decompress(&input_file, &output_file))
    }

    // For now, only checking that there is a second arg.

    /*
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
    output.write_all(&bytes).unwrap(); */
}


// ****** FILE COMPRESSOR ****** //

// Returns exit status of program
fn compress(input_filename: &str, output_filename: &str) -> i32 {
    0
}


// ****** FILE DECOMPRESSOR ****** //

// Returns exit status of program
fn decompress(input_filename: &str, output_filename: &str) -> i32 {
    0
}




// ****** ARGUMENT CHECKERS ****** //

// Parses args.
// Grabs the input and output filenames, if applicable.
// Grabs whether the input file is being zipped or unzipped.
// Return either the exit code the program should give, or none.
fn parse_args(input_filename: &mut Option<String>,
              output_filename: &mut Option<String>,
              zip: &mut bool,
              unzip: &mut bool) -> Option<i32> {

    let args: Vec<String> = env::args().collect();
    let mut ops = getopt::Parser::new(&args, "uzxi:o:");
    loop {
        match ops.next().transpose() {
            Ok(result) => match result {
                None => break,
                Some(opt) => match opt {
                    Opt('z', None)             => { *zip = true; }
                    Opt('x', None)             => { *unzip = true; }
                    Opt('i', Some(str)) => { *input_filename = Some(str.clone()); }
                    Opt('o', Some(str)) => { *output_filename = Some(str.clone()); }
                    Opt('u', None) => {
                        usage();
                        return Some(0);
                    }
                    _ => {
                        usage();
                        return Some(1);
                    }
                }
            }
            Err(_) => {
                usage();
                return Some(1);
            }
        }
    }
    None
}

// Check whether the provided combination of arguments is valid.
// If so, return none, indicating no exit code.
// If the args are invalid, return the program's exit code.
fn validate_args(input_filename: &Option<String>,
                 output_filename: &Option<String>,
                 zip: &bool,
                 unzip: &bool) -> Option<i32> {
    if input_filename.is_none() {
        println!("Input file not specified!");
        usage();
        return Some(1)
    }
    if output_filename.is_none() {
        println!("Output file not specified!");
        usage();
        return Some(1)
    }
    if zip == unzip {
        println!("Must either zip or unzip a file!");
        usage();
        return Some(1)
    }
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
