use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage();
        process::exit(1); 
    }
}

fn usage() {
    println!("Usage: wz [filename]");
}
