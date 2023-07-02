use std::fmt::Display;

use nom::{
    bytes::complete::tag,
    character::complete::multispace0 as multispace,
    combinator::map,
    sequence::{delimited, tuple},
    IResult, Parser,
};

use crate::parser::expr::{expr, identifier, Expr, Value};

pub struct Bind {
    pub name: String,
    // todo : add type check
    pub value: Expr,
}

impl Display for Bind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {} = {}", self.name, self.value)
    }
}

// let a = 123
pub fn bind(i: &str) -> IResult<&str, Bind> {
    map(
        tuple((
            tag("let"),
            identifier,
            delimited(multispace, tag("="), multispace),
            expr,
        )),
        |(_, id, _, expr)| match id {
            Expr::Factor(Value::Variable(s)) => Bind {
                name: s,
                value: expr,
            },
            _ => unreachable!(),
        },
    )
    .parse(i)
}

#[test]
fn bind_test() {
    assert_eq!(
        bind("let a = 123").map(|(i, b)| (i, format!("{}", b))),
        Ok(("", "let a = 123".to_string()))
    )
}

pub struct Set {
    pub name: String,
    pub value: Expr,
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}

// a = 123
pub fn set(i: &str) -> IResult<&str, Set> {
    map(
        tuple((
            identifier,
            delimited(multispace, tag("="), multispace),
            expr,
        )),
        |(id, _, expr)| match id {
            Expr::Factor(Value::Variable(s)) => Set {
                name: s,
                value: expr,
            },
            _ => unreachable!(),
        },
    )
    .parse(i)
}
#[test]
fn set_test() {
    assert_eq!(
        set("a = 123").map(|(i, b)| (i, format!("{}", b))),
        Ok(("", "a = 123".to_string()))
    )
}
