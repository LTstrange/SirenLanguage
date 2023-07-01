use std::fmt::Display;

use crate::parser::bind;
use crate::parser::expr;

use nom::{branch::alt, combinator::map, IResult};

pub enum Statement {
    Bind(bind::Bind),
    Expr(expr::Expr),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Bind(bind) => write!(f, "Bind: {}", bind),
            Statement::Expr(expr) => write!(f, "Expr: {}", expr),
        }
    }
}

pub fn statement(i: &str) -> IResult<&str, Statement> {
    alt((
        map(bind::bind, Statement::Bind),
        map(expr::expr, Statement::Expr),
    ))(i)
}
