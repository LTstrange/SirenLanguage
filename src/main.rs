use clap::Parser;
use colored::Colorize;
use siren_language::{run_file, run_line};
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

#[derive(Parser)]
struct Cli {
    #[arg(
        value_name = "source file",
        help = "Path to the source file to interpret"
    )]
    file: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(file) = cli.file {
        file_interpreter(file);
    } else {
        repl();
    }
}

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        // get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // parse and run input
        match input {
            "" => continue,
            "quit" | "q" => break,
            // run input on the environment
            input => {
                if let Err(msg) = run_line(input) {
                    println!("{}", format!("Error: {}", msg).red());
                }
            }
        }
    }
}

fn file_interpreter(path: PathBuf) {
    match fs::read_to_string(path.clone()) {
        Ok(content) => {
            if let Err(msg) = run_file(&content) {
                println!("{}", format!("Error: {}", msg).red());
            }
        }
        Err(e) => println!(
            "{}\n{}",
            e.to_string().red(),
            format!("Path: {:?}", path).red()
        ),
    }
}
