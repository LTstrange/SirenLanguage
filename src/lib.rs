mod parser;
pub use parser::SirenParser;
mod interpreter;
pub use interpreter::Environment;

// run a line of code on the given environment, and return the result
pub fn run(env: &mut Environment, input: &str) -> Result<Option<i64>, String> {
    let stmt = SirenParser::parse(input)?; // parse the input
    env.eval(stmt) // evaluate the input
}
