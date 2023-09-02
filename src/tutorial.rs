use std::io::{BufReader, BufRead};
use anyhow::{Context, Result};
use std::fs::File;
use clap::Parser;
use log::{info, warn};
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Cli::parse();
    let file = File::open(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
    info!("Reading the file.");
    let content = BufReader::new(file);
    for line in content.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);
            },
            Err(error) => { return {
                Err(error.into())}; }
        }
    }
    warn!("oops, issue happened while reading the file.");
    Ok(())
}

fn answer() -> i32 {
    42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}