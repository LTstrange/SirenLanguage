// mod interpreter;
mod parser;

// pub use interpreter::Evaluator;
// use interpreter::Value;

// // run a line of code on the given environment, and return the result
// pub fn run(evaluator: &mut Evaluator, input: &str) -> Result<Value, String> {
//     let stmt = parser::parse_line(input)?; // parse the input
//     evaluator.eval(&stmt) // evaluate the input
// }

// // run a file of code on the given environment, and return the result
// pub fn run_file(evaluator: &mut Evaluator, content: String) -> Result<(), String> {
//     let program = parser::parse_file(&content)?; // parse the input
//     for stmt in program.0 {
//         evaluator.eval(&stmt)?; // evaluate the input
//     }
//     Ok(())
// }

pub fn run_file(input: &str) -> Result<(), String> {
    let program = parser::parse_file(input).map_err(|e| format!("Parse error: {}", e))?;
    for item in program.0 {
        println!("{:?}", item);
    }
    Ok(())
}
