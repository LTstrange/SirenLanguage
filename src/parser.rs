use statement::{statement, Statement};

pub mod bind;
pub mod expr;
pub mod statement;

pub struct SirenParser;
impl SirenParser {
    pub fn parse(_input: &str) -> Result<Statement, String> {
        // use statement parser to parse one line code
        match statement(_input) {
            Ok((i, statement)) if i.is_empty() => Ok(statement), // success only if the input has reached the end
            Ok((_, _)) => Err(String::from("Invalid input")),
            Err(err) => Err(format!("{:?}", err)), // return the parse error
        }
    }
}
