use clap::Parser;
use colored::Colorize;
use siren_language::{run_file, SirenError};
use std::{fs, path::PathBuf};

#[derive(Parser)]
struct Cli {
    #[arg(
        value_name = "source file",
        help = "Path to the source file to interpret"
    )]
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    file_interpreter(cli.file);
}

fn file_interpreter(path: PathBuf) {
    match fs::read_to_string(path.clone()) {
        Ok(content) => {
            if let Err(msg) = run_file(&content) {
                match msg {
                    SirenError::Parse(msg) => println!("Parse error:\n{}", msg),
                    SirenError::Compile(msg) => println!("Compilation error:\n{}", msg),
                    SirenError::Runtime(msg) => println!("Runtime error:\n{}", msg),
                }
            }
        }
        Err(e) => println!(
            "{}\n{}",
            e.to_string().red(),
            format!("Path: {:?}", path).red()
        ),
    }
}
