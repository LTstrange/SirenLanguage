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

#[test]
fn statement_test() {
    assert_eq!(
        statement("let a = 123").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", "Bind: let a = 123".to_string()))
    );
    assert_eq!(
        statement("123 + 254").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", "Expr: (123 + 254)".to_string()))
    );
    assert_eq!(
        statement("let abc =123 + 254").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", "Bind: let abc = (123 + 254)".to_string()))
    );
}
