use colored::Colorize;
use siren_language::{run, run_file, Environment};
use std::io::{self, Read, Write};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    // Create a new running environment
    let mut env = Environment::new();

    if args.len() == 1 {
        repl(&mut env);
    } else if args.len() == 2 && args[1].ends_with(".siren") {
        // file interprete
        file_interpreter(&mut env, &args[1]);
    } else {
        println!("Usage:");
        println!("    siren              : repl");
        println!("    siren <file>.siren : interpret file");
    }
}

fn repl(env: &mut Environment) {
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
            input => match run(env, input) {
                Ok(output) => match output {
                    Some(number) => println!("{}", number), // print the result
                    None => continue, // print nothing because of let statement
                },
                Err(msg) => println!("{}", format!("Error: {}", msg).red()), // print error
            },
        }
    }
}

fn file_interpreter(env: &mut Environment, file_name: &str) {
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("Content:");
    println!("{}", content);
    match run_file(env, content) {
        Ok(()) => (),
        Err(msg) => {
            println!("{}", format!("Error: {}", msg).red());
            return;
        } // print error
    }
    println!("Done.");
    println!("Env:"); // print variables in the environment
    println!("{}", env);
}
