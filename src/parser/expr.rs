use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use std::str::FromStr;

use nom::character::complete::alpha1;
use nom::Parser;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1 as digit, multispace0 as multispace},
    combinator::{map, map_res},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

pub enum Value {
    Value(i64),
    Variable(String),
}

pub enum Expr {
    Factor(Value),
    UnExpr(Prefix, Box<Expr>),
    BinExpr(Box<Expr>, Infix, Box<Expr>),
}

// impl Expr {
//     pub fn eval(&self) -> i64 {
//         match self {
//             Expr::Factor(v) => match v {
//                 Value::Value(n) => *n,
//                 Value::Variable(_v) => todo!(),
//             },
//             Expr::UnExpr(op, right) => match op {
//                 Prefix::Minus => -right.eval(),
//             },
//             Expr::BinExpr(left, op, right) => match op {
//                 Infix::Add => left.eval() + right.eval(),
//                 Infix::Sub => left.eval() - right.eval(),
//                 Infix::Mul => left.eval() * right.eval(),
//                 Infix::Div => left.eval() / right.eval(),
//             },
//         }
//     }
// }

pub enum Prefix {
    Minus,
}

#[derive(Debug)]
pub enum Infix {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for Expr {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        use self::Expr::*;
        match self {
            Factor(val) => match val {
                Value::Value(n) => write!(format, "{}", n),
                Value::Variable(v) => write!(format, "{}", v),
            },
            UnExpr(op, right) => match op {
                Prefix::Minus => write!(format, "(-{})", right),
            },
            BinExpr(left, op, right) => match op {
                Infix::Add => write!(format, "({} + {})", left, right),
                Infix::Sub => write!(format, "({} - {})", left, right),
                Infix::Mul => write!(format, "({} * {})", left, right),
                Infix::Div => write!(format, "({} / {})", left, right),
            },
        }
    }
}

pub fn identifier(i: &str) -> IResult<&str, Expr> {
    // variable
    map(delimited(multispace, alpha1, multispace), |s: &str| {
        Expr::Factor(Value::Variable(s.to_string()))
    })
    .parse(i)
}

fn number(i: &str) -> IResult<&str, Expr> {
    map(
        map_res(delimited(multispace, digit, multispace), FromStr::from_str),
        |n: i64| Expr::Factor(Value::Value(n)),
    )
    .parse(i)
}

fn parens(i: &str) -> IResult<&str, Expr> {
    delimited(multispace, delimited(tag("("), expr, tag(")")), multispace).parse(i)
}

fn factor(i: &str) -> IResult<&str, Expr> {
    alt((
        identifier,
        number,
        map(
            delimited(
                multispace,
                preceded(tag("-"), alt((number, parens, identifier))),
                multispace,
            ),
            |a| Expr::UnExpr(Prefix::Minus, Box::new(a)),
        ),
        parens,
    ))
    .parse(i)
}

fn fold_exprs(initial: Expr, remainder: Vec<(Infix, Expr)>) -> Expr {
    remainder.into_iter().fold(initial, |acc, pair| {
        let (oper, expr) = pair;
        Expr::BinExpr(Box::new(acc), oper, Box::new(expr))
    })
}

fn term(i: &str) -> IResult<&str, Expr> {
    let (i, initial) = factor(i)?;
    let (i, remainder) = many0(alt((
        |i| {
            let (i, mul) = preceded(tag("*"), factor).parse(i)?;
            Ok((i, (Infix::Mul, mul)))
        },
        |i| {
            let (i, div) = preceded(tag("/"), factor).parse(i)?;
            Ok((i, (Infix::Div, div)))
        },
    )))
    .parse(i)?;

    Ok((i, fold_exprs(initial, remainder)))
}

pub fn expr(i: &str) -> IResult<&str, Expr> {
    let (i, initial) = term(i)?;
    let (i, remainder) = many0(alt((
        |i| {
            let (i, add) = preceded(tag("+"), term).parse(i)?;
            Ok((i, (Infix::Add, add)))
        },
        |i| {
            let (i, sub) = preceded(tag("-"), term).parse(i)?;
            Ok((i, (Infix::Sub, sub)))
        },
    )))
    .parse(i)?;

    Ok((i, fold_exprs(initial, remainder)))
}

#[test]
fn factor_test() {
    assert_eq!(
        factor("  3  ").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("3")))
    );
}

#[test]
fn term_test() {
    assert_eq!(
        term(" 3 *  5   ").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("(3 * 5)")))
    );
}

#[test]
fn expr_test() {
    assert_eq!(
        expr(" 1 + 2 *  3 ").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("(1 + (2 * 3))")))
    );
    assert_eq!(
        expr(" 1 + 2 *  3 / 4 - 5 ").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("((1 + ((2 * 3) / 4)) - 5)")))
    );
    assert_eq!(
        expr(" 72 / 2 / 3 ").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("((72 / 2) / 3)")))
    );
}

#[test]
fn parens_test() {
    assert_eq!(
        expr(" ( 1 + 2 ) *  3 ").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("((1 + 2) * 3)")))
    );
}

#[test]
fn unary_test() {
    assert_eq!(
        expr(" - 1 ").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("(-1)")))
    );
    assert_eq!(
        expr("2 * -1").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("(2 * (-1))")))
    );
    assert_eq!(
        expr("-(2 * 1)").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("(-(2 * 1))")))
    );
    assert_eq!(
        expr("-1 + 3").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("((-1) + 3)")))
    );
}

#[test]
fn identifier_test() {
    assert_eq!(
        expr("x").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("x")))
    );
    assert_eq!(
        expr("2 + x").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("(2 + x)")))
    );
    assert_eq!(
        expr("-x").map(|(i, x)| (i, format!("{}", x))),
        Ok(("", String::from("(-x)")))
    );
}