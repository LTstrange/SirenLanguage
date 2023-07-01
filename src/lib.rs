use statement::{statement, Statement};

mod bind;
mod expr;
mod statement;

pub struct SirenParser;
impl SirenParser {
    pub fn parse(_input: &str) -> Result<Statement, String> {
        match statement(_input) {
            Ok((i, statement)) if i.is_empty() => Ok(statement),
            Ok((_, _)) => Err(String::from("Invalid input")),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
