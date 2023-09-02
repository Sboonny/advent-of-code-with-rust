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

fn read_file(content: BufReader<File>, mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    for line in content.lines() {
        match line {
            Ok(line) => {
                writeln!(writer, "{}", line).ok();
            },
            Err(error) => { return {
                Err(error.into())}; }
        }
    }
    warn!("oops, issue happened while reading the file.");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Cli::parse();
    let file = File::open(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
    info!("Reading the file.");
    let content = BufReader::new(file);
    read_file(content, &mut std::io::stdout())
}


// ToDo: figure out how to read a file in a test and expect its context.
// #[test]
// fn file_is_read() {
//     let mut result = Vec::new();
//     let path = "src/tutorial.rs";
//     let file = File::open(path);
//     let content = BufReader::new(file);
//     read_file(content, &mut result);
//     assert_eq!(result, b"lorem ipsum\n");
// }