mod bind;
mod expr;
use std::fmt::Display;

use bind::Bind;
use expr::*;
use nom::{branch::alt, combinator::map, IResult};

pub enum Statement {
    Bind(Bind),
    Expr(Expr),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Bind(bind) => write!(f, "Bind: {}", bind),
            Statement::Expr(expr) => write!(f, "Expr: {}", expr),
        }
    }
}

fn statement(i: &str) -> IResult<&str, Statement> {
    alt((
        map(bind::bind, Statement::Bind),
        map(expr::expr, Statement::Expr),
    ))(i)
}

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
