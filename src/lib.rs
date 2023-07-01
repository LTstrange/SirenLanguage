mod parser;
pub use parser::SirenParser;
mod interpreter;
pub use interpreter::Environment;

pub fn run(env: &mut Environment, input: &str) -> Result<Option<i64>, String> {
    let stmt = SirenParser::parse(input)?;
    env.eval(stmt)
}
