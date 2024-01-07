use std::env;
use std::process;
use std::fs;

mod tree {
    pub mod node;
}

mod encoding;
mod freq;

fn main() {
    // For now, only checking that there is a second arg.
    // Not complaining if they pass too few.
    let filename: String = match env::args().skip(1).next() {
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

    let freq = freq::gen_frequency(&bytes);
    println!("{:?}", encoding::gen_encoding(&freq));
    println!("{:?}", freq::normalize(&freq));
}
