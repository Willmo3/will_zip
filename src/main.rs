use std::env;
use std::io;
use std::process;
use std::fs;

fn main() -> io::Result<()> {
    let filename: String = match get_filename() {
        Ok(name) => name,
        Err(message) => {
            println!("{}", message);
            process::exit(1)
        },
    };

    let bytes :Vec<u8> = match fs::read(&filename) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("File not found: {}", &filename);
            process::exit(1)
        },
    };

    println!("{:?}", bytes);
    // Read in file
    Ok(())
}

fn get_filename() -> Result<String, &'static str> {
    match env::args().skip(1).next() {
        None => Err("File name not specified! Usage: wz [filename]"),
        Some(filename) => Ok(filename),
    }
}
