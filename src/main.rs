use clap::{Parser, Subcommand};
use colored::Colorize;
use siren_language::{parse_file, pretty_print_program, run_file, SirenError};
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
        #[arg(short, long, help = "Pretty print the AST")]
        pretty: bool,
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
        Command::Parse { pretty, file } => print_ast(pretty, file),
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

fn print_ast(pretty: bool, file: PathBuf) {
    match fs::read_to_string(file.clone()) {
        Ok(content) => match (pretty, parse_file(&content)) {
            (false, Ok(p)) => println!("{}", p),
            (true, Ok(p)) => pretty_print_program(&p, 0),
            (_, Err(msg)) => println!("Parse error:\n{}", msg),
        },
        Err(e) => println!(
            "{}\n{}",
            e.to_string().red(),
            format!("Path: {:?}", file).red()
        ),
    }
}
