// #![allow(unused)]

use nom::bytes::complete::take;
use nom::combinator::opt;
use nom::error::{Error, ErrorKind};
use nom::sequence::{terminated, tuple};
use nom::{
    branch::alt,
    combinator::{map, verify},
    multi::many0,
    sequence::{delimited, pair, preceded},
    Err, IResult,
};

use super::ast::*;
use crate::lexer::{Token, Tokens};

macro_rules! tag_token (
    ($func_name:ident, $tag: expr) => (
        fn $func_name(tokens: Tokens) -> IResult<Tokens, Tokens> {
            verify(take(1usize), |t: &Tokens| t.tok[0] == $tag)(tokens)
        }
    )
);

tag_token!(let_tag, Token::Let);
tag_token!(assign_tag, Token::Assign);
tag_token!(semicolon_tag, Token::Semicolon);
tag_token!(return_tag, Token::Return);
tag_token!(lparen_tag, Token::LParen);
tag_token!(rparen_tag, Token::RParen);
tag_token!(lbracket_tag, Token::LBracket);
tag_token!(rbracket_tag, Token::RBracket);
tag_token!(lbrace_tag, Token::LBrace);
tag_token!(rbrace_tag, Token::RBrace);
tag_token!(comma_tag, Token::Comma);
// tag_token!(colon_tag, Token::Colon);
tag_token!(sub_tag, Token::Sub);
tag_token!(not_tag, Token::Not);
tag_token!(if_tag, Token::If);
tag_token!(else_tag, Token::Else);
tag_token!(fn_tag, Token::Fn);

fn infix_op(tok: &Token) -> (Prec, Option<Infix>) {
    match tok {
        Token::Add => (Prec::Sum, Some(Infix::Add)),
        Token::Sub => (Prec::Sum, Some(Infix::Sub)),
        Token::Mul => (Prec::Product, Some(Infix::Mul)),
        Token::Div => (Prec::Product, Some(Infix::Div)),
        Token::Equal => (Prec::Equals, Some(Infix::Eql)),
        Token::NotEqual => (Prec::Equals, Some(Infix::Neq)),
        Token::Less => (Prec::LessGreater, Some(Infix::Lt)),
        Token::LessEqual => (Prec::LessGreater, Some(Infix::Lte)),
        Token::Greater => (Prec::LessGreater, Some(Infix::Gt)),
        Token::GreaterEqual => (Prec::LessGreater, Some(Infix::Gte)),
        Token::LParen => (Prec::Call, None),
        Token::LBracket => (Prec::Index, None),
        _ => (Prec::Lowest, None),
    }
}

// int bool
fn literal(input: Tokens) -> IResult<Tokens, Expr> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        match t1.tok[0] {
            Token::Int(n) => Ok((i1, Expr::Literal(Literal::Int(n)))),
            Token::Bool(b) => Ok((i1, Expr::Literal(Literal::Bool(b)))),
            _ => Err(Err::Error(Error::new(input, ErrorKind::Tag))),
        }
    }
}

fn identity(input: Tokens) -> IResult<Tokens, Expr> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        match &t1.tok[0] {
            Token::Ident(s) => Ok((i1, Expr::Ident(s.to_string()))),
            _ => Err(Err::Error(Error::new(input, ErrorKind::Tag))),
        }
    }
}

fn prefix_expr(input: Tokens) -> IResult<Tokens, Expr> {
    let (input, prefix) = alt((sub_tag, not_tag))(input)?;
    if prefix.tok.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        let (input, oprand) = atom_expr(input)?;
        match &prefix.tok[0] {
            Token::Sub => Ok((input, Expr::UnExpr(Prefix::Minus, Box::new(oprand)))),
            Token::Not => Ok((input, Expr::UnExpr(Prefix::Not, Box::new(oprand)))),
            _ => Err(Err::Error(Error::new(input, ErrorKind::Tag))),
        }
    }
}

fn parent_expr(input: Tokens) -> IResult<Tokens, Expr> {
    delimited(lparen_tag, expr, rparen_tag)(input)
}

fn parse_params(input: Tokens) -> IResult<Tokens, Vec<Expr>> {
    map(
        pair(identity, many0(preceded(comma_tag, identity))),
        |(p, ps)| [&vec![p][..], &ps[..]].concat(),
    )(input)
}

fn fn_expr(input: Tokens) -> IResult<Tokens, Expr> {
    map(
        tuple((
            fn_tag,
            lparen_tag,
            alt((parse_params, empty)),
            rparen_tag,
            block_stmt,
        )),
        |(_, _, p, _, b)| {
            let params = p
                .iter()
                .map(|e| match e {
                    Expr::Ident(name) => name.to_string(),
                    _ => unreachable!(),
                })
                .collect();
            Expr::Function { params, body: b }
        },
    )(input)
}

fn parse_comma_exprs(input: Tokens) -> IResult<Tokens, Expr> {
    preceded(comma_tag, expr)(input)
}
fn parse_exprs(input: Tokens) -> IResult<Tokens, Vec<Expr>> {
    map(pair(expr, many0(parse_comma_exprs)), |(first, second)| {
        [&vec![first][..], &second[..]].concat()
    })(input)
}
fn empty(input: Tokens) -> IResult<Tokens, Vec<Expr>> {
    Ok((input, vec![]))
}
fn call_expr(input: Tokens, func: Expr) -> IResult<Tokens, Expr> {
    map(
        delimited(lparen_tag, alt((parse_exprs, empty)), rparen_tag),
        |e| Expr::Call {
            func: Box::new(func.clone()),
            args: e,
        },
    )(input)
}

fn index_expr(input: Tokens, arr: Expr) -> IResult<Tokens, Expr> {
    map(delimited(lbracket_tag, expr, rbracket_tag), |idx| {
        Expr::Index {
            arr: Box::new(arr.clone()),
            index: Box::new(idx),
        }
    })(input)
}

fn infix_expr(input: Tokens, left: Expr) -> IResult<Tokens, Expr> {
    let (input, tok) = take(1usize)(input)?;
    let (prec, op) = infix_op(&tok.tok[0]);
    let op = op.unwrap();
    let (input, right) = pratt_expr(input, prec)?;
    Ok((input, Expr::BinExpr(Box::new(left), op, Box::new(right))))
}

fn atom_expr(input: Tokens) -> IResult<Tokens, Expr> {
    alt((literal, identity, prefix_expr, parent_expr, fn_expr))(input)
}

fn pratt_expr(input: Tokens, prec: Prec) -> IResult<Tokens, Expr> {
    let (mut input, mut left) = atom_expr(input)?;
    loop {
        if input.tok.is_empty() {
            return Ok((input, left));
        }
        let p = take(1usize)(input).map(|(_, t)| infix_op(&t.tok[0]))?;
        if p.0 <= prec {
            break;
        }
        match p {
            (Prec::Call, _) => {
                (input, left) = call_expr(input, left)?;
            }
            (Prec::Index, _) => {
                (input, left) = index_expr(input, left)?;
            }
            (_, _) => {
                (input, left) = infix_expr(input, left)?;
            }
        }
    }
    Ok((input, left))
}

fn expr(input: Tokens) -> IResult<Tokens, Expr> {
    pratt_expr(input, Prec::Lowest)
}

fn expr_stmt(input: Tokens) -> IResult<Tokens, Statement> {
    map(expr, Statement::Expr)(input)
}

fn return_stmt(input: Tokens) -> IResult<Tokens, Statement> {
    map(preceded(return_tag, expr), Statement::Return)(input)
}

fn let_stmt(input: Tokens) -> IResult<Tokens, Statement> {
    map(
        tuple((let_tag, identity, assign_tag, expr)),
        |(_, ident, _, value)| match ident {
            Expr::Ident(name) => Statement::Let { name, value },
            _ => unreachable!(),
        },
    )(input)
}

pub fn statement(input: Tokens) -> IResult<Tokens, Statement> {
    alt((expr_stmt, return_stmt, let_stmt))(input)
}

fn block_stmt(input: Tokens) -> IResult<Tokens, Vec<Statement>> {
    map(
        delimited(
            lbrace_tag,
            tuple((many0(terminated(statement, semicolon_tag)), opt(statement))),
            rbrace_tag,
        ),
        |(mut stmts, ret)| {
            if let Some(Statement::Expr(expr)) = ret {
                stmts.push(Statement::Return(expr));
            }
            stmts
        },
    )(input)
}

pub fn program(input: Tokens) -> IResult<Tokens, Program> {
    many0(terminated(statement, semicolon_tag))(input)
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::lexer::{lexer, Token};

    #[test]
    fn test_parse_expr() {
        let tokens: Vec<Token> = lexer("1 + 2 * abc + 4 * 5 - 6 / 7");
        let tokens = Tokens::new(&tokens);
        assert_eq!(
            expr(tokens).map(|(_, x)| format!("{:?}", x)),
            Ok("(((1 + (2 * abc)) + (4 * 5)) - (6 / 7))".to_string())
        );
    }

    #[test]
    fn test_prefix_expr() {
        let tokens: Vec<Token> = lexer("1 + -2 * abc");
        let tokens = Tokens::new(&tokens);
        assert_eq!(
            expr(tokens).map(|(_, x)| format!("{:?}", x)),
            Ok("(1 + ((-2) * abc))".to_string())
        )
    }

    #[test]
    fn test_parent_expr() {
        let tokens = lexer("(1 + 2) * 3");
        let tokens = Tokens::new(&tokens);
        assert_eq!(
            expr(tokens).map(|(_, x)| format!("{:?}", x)),
            Ok("((1 + 2) * 3)".to_string())
        )
    }
}
