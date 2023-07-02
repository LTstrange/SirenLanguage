use std::fmt::Display;

use crate::parser::expr;
use crate::parser::variable;

use nom::{branch::alt, combinator::map, IResult};

pub enum Statement {
    Bind(variable::Bind),
    Expr(expr::Expr),
    Set(variable::Set),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Bind(bind) => write!(f, "Bind: {}", bind),
            Statement::Expr(expr) => write!(f, "Expr: {}", expr),
            Statement::Set(set) => write!(f, "Set: {}", set),
        }
    }
}

// oneline code parser
pub fn statement(i: &str) -> IResult<&str, Statement> {
    alt((
        map(variable::set, Statement::Set),   // set: "a = 123"
        map(variable::bind, Statement::Bind), // bind: "let a = 123"
        map(expr::expr, Statement::Expr),     // expr: "(123 + 234) / 5"
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
    assert_eq!(
        statement("abc =123 + 254").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", "Set: abc = (123 + 254)".to_string()))
    );
}
