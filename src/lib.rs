mod parser;
pub use parser::SirenParser;
mod interpreter;
pub use interpreter::Environment;

// run a line of code on the given environment, and return the result
pub fn run(env: &mut Environment, input: &str) -> Result<Option<i64>, String> {
    let stmt = SirenParser::parse_line(input)?; // parse the input
    env.eval(stmt) // evaluate the input
}

// run a file of code on the given environment, and return the result
pub fn run_file(env: &mut Environment, content: String) -> Result<(), String> {
    let stmts = SirenParser::parse_file(&content)?; // parse the input
    for stmt in stmts {
        env.eval(stmt)?; // evaluate the input
    }
    Ok(())
}
