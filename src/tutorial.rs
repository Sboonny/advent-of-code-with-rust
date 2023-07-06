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
    let content = BufReader::new(file);
    for line in content.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);
            },
            _ => {}
        }
    }
}
