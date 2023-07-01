use colored::Colorize;
use siren_language::{run, Environment};
use std::io::{self, Write};

fn main() {
    let mut env = Environment::new();
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
            input => match run(&mut env, input) {
                Ok(output) => match output {
                    Some(number) => println!("{}", number),
                    None => continue,
                },
                Err(msg) => println!("{}", format!("Error: {}", msg).red()),
            },
        }
    }
}
