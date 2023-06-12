use std::io::{BufReader, BufRead};
use std::fs::File;
use clap::Parser;
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = File::open(&args.path).expect("could not read file");
    let mut content = BufReader::new(file);
    let mut line =  String::new();
    let len = content.read_line(&mut line); 
    println!("here is the length: {:?}", len);
}
