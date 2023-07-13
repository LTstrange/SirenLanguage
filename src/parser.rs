pub mod parse_func;

mod ast;
pub use ast::*;
use nom::InputLength;
pub use parse_func::*;

use crate::lexer::{lexer, Tokens};

pub struct SirenParser;
impl SirenParser {
    pub fn parse_file(input: &str) -> Result<Program, String> {
        let lexer = lexer(input);
        let tokens = Tokens::new(&lexer);

        match program(tokens) {
            Ok((i, stmts)) if i.input_len() == 0 => Ok(stmts),
            Ok((i, _)) => Err(format!("Here has something left to parse: {:?}", i)),
            Err(err) => Err(format!("{:?}", err)), // return the parse error
        }
    }

    pub fn parse_line(input: &str) -> Result<Statement, String> {
        let lexer = lexer(input);
        let tokens = Tokens::new(&lexer);
        // use statement parser to parse one line code
        match statement(tokens) {
            Ok((i, statement)) if i.input_len() == 0 => Ok(statement), // success only if the input has reached the end
            Ok((i, _)) => Err(format!("Invalid input; rest {:?}", i)),
            Err(err) => Err(format!("{:?}", err)), // return the parse error
        }
    }
}

// todo : pratt parser
