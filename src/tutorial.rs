use std::io::{BufReader, BufRead};
use anyhow::{Context, Result};
use std::fs::File;
use clap::Parser;
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let file = File::open(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let content = BufReader::new(file);
    for line in content.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);
            },
            Err(error) => { return Err(error.into()); }
        }
    }
    Ok(())
}
