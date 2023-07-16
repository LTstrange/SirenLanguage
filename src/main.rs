use colored::Colorize;
use siren_language::{run, run_file, Evaluator};
use std::io::{self, Read, Write};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    // Create a new running environment
    let mut env = Evaluator::new();

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

fn repl(evaluator: &mut Evaluator) {
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
            input => match run(evaluator, input) {
                Ok(output) => match output {
                    Some(number) => println!("{}", number), // print the result
                    None => continue, // print nothing because of let statement
                },
                Err(msg) => println!("{}", format!("Error: {}", msg).red()),
            },
        }
    }
}

fn file_interpreter(evaluator: &mut Evaluator, file_name: &str) {
    let file = std::fs::File::open(file_name);
    match file {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            println!("Content:");
            println!("{}", content);
            match run_file(evaluator, content) {
                Ok(()) => println!("Done."),
                Err(msg) => {
                    println!("{}", format!("Error: {}", msg).red());
                }
            }

            println!("Env:"); // print variables in the environment
            println!("{}", evaluator.env.borrow());
        }
        Err(msg) => {
            // not such file
            println!("{}", format!("Error: {}", msg).red());
        }
    }
}
