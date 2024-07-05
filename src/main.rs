use clap::{Parser, Subcommand};
use colored::Colorize;
use siren_language::{parse_file, run_file, SirenError};
use std::{fs, path::PathBuf};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(
            value_name = "source file",
            help = "Path to the source file to interpret"
        )]
        file: PathBuf,
    },
    Parse {
        #[arg(
            value_name = "source file",
            help = "Path to the source file to interpret"
        )]
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Run { file } => file_interpreter(file),
        Command::Parse { file } => print_ast(file),
    }
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

fn print_ast(file: PathBuf) {
    match fs::read_to_string(file.clone()) {
        Ok(content) => match parse_file(&content) {
            Ok(p) => println!("{}", p),
            Err(msg) => println!("Parse error:\n{}", msg),
        },
        Err(e) => println!(
            "{}\n{}",
            e.to_string().red(),
            format!("Path: {:?}", file).red()
        ),
    }
}
