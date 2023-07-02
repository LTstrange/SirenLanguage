use statement::{statement, Statement};

use self::statement::statements;

pub mod expr;
pub mod statement;
pub mod variable;

pub struct SirenParser;
impl SirenParser {
    pub fn parse_file(input: &str) -> Result<Vec<Statement>, String> {
        match statements(input) {
            Ok((i, stmts)) if i.is_empty() => Ok(stmts),
            Ok((i, _)) => unreachable!("Here has something left to parse: {}", i),
            Err(err) => Err(format!("{:?}", err)), // return the parse error
        }
    }

    pub fn parse_line(input: &str) -> Result<Statement, String> {
        // use statement parser to parse one line code
        match statement(input) {
            Ok((i, statement)) if i.is_empty() => Ok(statement), // success only if the input has reached the end
            Ok((_, _)) => Err(String::from("Invalid input")),
            Err(err) => Err(format!("{:?}", err)), // return the parse error
        }
    }
}
