use clap::Parser;
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let _args = Cli::parse();
    let args: Vec<String> = std::env::args().collect();

    println!("My path is {}.", args[0]);
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}
