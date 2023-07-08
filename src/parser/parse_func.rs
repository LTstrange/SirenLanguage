use std::str::FromStr;

use nom::character::complete::alpha1;
use nom::combinator::{opt, recognize};
use nom::multi::separated_list0;
use nom::sequence::tuple;
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

use super::ast::*;

pub fn identifier(i: &str) -> IResult<&str, Expr> {
    // variable
    map(delimited(multispace, alpha1, multispace), |s: &str| {
        Expr::Ident(s.to_string())
    })
    .parse(i)
}

fn number(i: &str) -> IResult<&str, Expr> {
    map(
        map_res(delimited(multispace, digit, multispace), FromStr::from_str),
        |n: i64| Expr::Literal(Literal::Int(n)),
    )
    .parse(i)
}

fn function(i: &str) -> IResult<&str, Expr> {
    // arguments
    let (i, params) = preceded(
        tuple((multispace, tag("fn"), multispace)),
        // tuple of arguments
        delimited(
            tag("("),
            separated_list0(tag(","), recognize(identifier)),
            tag(")"),
        ),
    )(i)?;
    // statements
    let (i, body) = delimited(
        tuple((multispace, tag("{"))),
        statements,
        tuple((multispace, tag("}"), multispace)),
    )(i)?;
    let params = params
        .into_iter()
        .map(|arg| arg.trim().to_string())
        .collect();
    Ok((i, Expr::Function { params, body }))
}

fn parens(i: &str) -> IResult<&str, Expr> {
    delimited(multispace, delimited(tag("("), expr, tag(")")), multispace).parse(i)
}

// conclude identifier, number, unary expression and parens
fn factor(i: &str) -> IResult<&str, Expr> {
    alt((
        function,
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

// fold list of expressions
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

// oneline code parser
pub fn statement(i: &str) -> IResult<&str, Statement> {
    alt((
        set,                        // set: "a = 123"
        bind,                       // bind: "let a = 123"
        map(expr, Statement::Expr), // expr: "(123 + 234) / 5"
    ))(i)
}

// multi line code parser: list of statement
pub fn statements(i: &str) -> IResult<&str, Vec<Statement>> {
    map(
        tuple((
            many0(delimited(multispace, statement, tag(";"))),
            opt(preceded(multispace, statement)),
        )),
        |(mut stmts, ret)| {
            if let Some(Statement::Expr(expr)) = ret {
                stmts.push(Statement::Return(expr));
            }
            stmts
        },
    )(i)
}

// let a = 123 : let statement
pub fn bind(i: &str) -> IResult<&str, Statement> {
    map(
        tuple((
            tag("let"),
            identifier,
            delimited(multispace, tag("="), multispace),
            expr,
        )),
        |(_, id, _, expr)| match id {
            Expr::Ident(s) => Statement::Let {
                name: s,
                value: expr,
            },
            _ => unreachable!(),
        },
    )
    .parse(i)
}

// a = 123
pub fn set(i: &str) -> IResult<&str, Statement> {
    map(
        tuple((
            identifier,
            delimited(multispace, tag("="), multispace),
            expr,
        )),
        |(id, _, expr)| match id {
            Expr::Ident(s) => Statement::Set {
                name: s,
                value: expr,
            },
            _ => unreachable!(),
        },
    )
    .parse(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn factor_test() {
        assert_eq!(
            factor("  3  ").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", "3".to_string()))
        );
    }

    #[test]
    fn term_test() {
        assert_eq!(
            term(" 3 *  5   ").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", "(3 * 5)".to_string()))
        );
    }

    #[test]
    fn expr_test() {
        assert_eq!(
            expr(" 1 + 2 *  3 ").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", "(1 + (2 * 3))".to_string()))
        );
        assert_eq!(
            expr(" 1 + 2 *  3 / 4 - 5 ").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", "((1 + ((2 * 3) / 4)) - 5)".to_string()))
        );
        assert_eq!(
            expr(" 72 / 2 / 3 ").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", String::from("((72 / 2) / 3)")))
        );
    }

    #[test]
    fn parens_test() {
        assert_eq!(
            expr(" ( 1 + 2 ) *  3 ").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", String::from("((1 + 2) * 3)")))
        );
    }

    #[test]
    fn unary_test() {
        assert_eq!(
            expr(" - 1 ").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", String::from("(-1)")))
        );
        assert_eq!(
            expr("2 * -1").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", String::from("(2 * (-1))")))
        );
        assert_eq!(
            expr("-(2 * 1)").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", String::from("(-(2 * 1))")))
        );
        assert_eq!(
            expr("-1 + 3").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", String::from("((-1) + 3)")))
        );
    }

    #[test]
    fn identifier_test() {
        assert_eq!(
            expr("x").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", String::from("x")))
        );
        assert_eq!(
            expr("2 + x").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", String::from("(2 + x)")))
        );
        assert_eq!(
            expr("-x").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", String::from("(-x)")))
        );
    }

    #[test]
    fn function_test() {
        assert_eq!(
            function("fn(x, y) { x + y;  x - y}").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", "fn(x, y) { expr (x + y); return (x - y); }".to_string()))
        );
    }

    #[test]
    fn statement_test() {
        assert_eq!(
            statement("let a = 123").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", "let a = 123".to_string()))
        );
        assert_eq!(
            statement("123 + 254  ").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", "expr (123 + 254)".to_string()))
        );
        assert_eq!(
            statement("let abc =123 + 254  ").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", "let abc = (123 + 254)".to_string()))
        );
        assert_eq!(
            statement("abc =123 + 254 ").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", "set abc = (123 + 254)".to_string()))
        );
        assert_eq!(
            statement("let abc = fn (a, b) {  a + b;}").map(|(i, x)| (i, format!("{:?}", x))),
            Ok(("", "let abc = fn(a, b) { expr (a + b); }".to_string()))
        );
    }

    #[test]
    fn statements_test() {
        assert_eq!(
            statements("  let a = 123 ;").map(|(i, stmts)| {
                (
                    i,
                    stmts
                        .iter()
                        .map(|stmt| format!("{:?}", stmt))
                        .collect::<Vec<String>>(),
                )
            }),
            Ok(("", vec!["let a = 123".to_string(),],)),
        );
        assert_eq!(
            statements("  let a = 123 ;   123 - 12 / 4  ; a= b  ;").map(|(i, stmts)| {
                (
                    i,
                    stmts
                        .iter()
                        .map(|stmt| format!("{:?}", stmt))
                        .collect::<Vec<String>>(),
                )
            }),
            Ok((
                "",
                vec![
                    "let a = 123".to_string(),
                    "expr (123 - (12 / 4))".to_string(),
                    "set a = b".to_string()
                ],
            )),
        );
    }

    #[test]
    fn bind_test() {
        assert_eq!(
            bind("let a = 123").map(|(i, b)| (i, format!("{:?}", b))),
            Ok(("", "let a = 123".to_string()))
        )
    }
    #[test]
    fn set_test() {
        assert_eq!(
            set("a = 123").map(|(i, b)| (i, format!("{:?}", b))),
            Ok(("", "set a = 123".to_string()))
        )
    }
}
