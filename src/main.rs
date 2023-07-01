use colored::Colorize;
use siren_language::SirenParser;
use std::io::{self, Write};

fn main() {
    // repl
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "" => continue,
            "quit" | "q" => break,
            input => match SirenParser::parse(input) {
                Ok(expr) => println!("{}", expr),
                Err(msg) => println!("{}", format!("Error: {}", msg).red()),
            },
        }
    }
}
