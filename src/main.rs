use colored::Colorize;
use siren_language::{run, Environment};
use std::io::{self, Write};

fn main() {
    // Create a new running environment
    let mut env = Environment::new();
    // repl
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
            input => match run(&mut env, input) {
                Ok(output) => match output {
                    Some(number) => println!("{}", number), // print the result
                    None => continue, // print nothing because of let statement
                },
                Err(msg) => println!("{}", format!("Error: {}", msg).red()), // print error
            },
        }
    }
}
