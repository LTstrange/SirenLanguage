mod ast;

use ast::*;
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct SirenParser;

fn build_pratt_parser() -> PrattParser<Rule> {
    PrattParser::new()
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
        .op(Op::prefix(Rule::neg))
        .op(Op::postfix(Rule::call))
}

fn pratt_parse<'a>(expr: Pairs<'a, Rule>, pratt: &PrattParser<Rule>) -> Expr<'a> {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::ident => Expr::Ident(primary.as_str()),
            Rule::number => Expr::Literal(Literal::Number(primary.as_str().parse().unwrap())),
            Rule::boolean => Expr::Literal(Literal::Boolean(primary.as_str().parse().unwrap())),
            p => unreachable!("get unexpected primary in pratt: {p:?}"),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => Expr::BinOp(Box::new(lhs), Infix::Add, Box::new(rhs)),
            Rule::sub => Expr::BinOp(Box::new(lhs), Infix::Sub, Box::new(rhs)),
            Rule::mul => Expr::BinOp(Box::new(lhs), Infix::Mul, Box::new(rhs)),
            Rule::div => Expr::BinOp(Box::new(lhs), Infix::Div, Box::new(rhs)),
            _ => unreachable!("get unexpected infix operator in pratt: {op:?}"),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::neg => Expr::UnaryOp(Prefix::Neg, Box::new(rhs)),
            _ => unreachable!("get unexpected prefix operator in pratt: {op:?}"),
        })
        .parse(expr)
}

fn parse_item<'a>(item: Pair<'a, Rule>, pratt: &PrattParser<Rule>) -> Option<Item<'a>> {
    match item.as_rule() {
        Rule::let_stmt => {
            let mut pairs = item.into_inner();
            let ident = pairs.next().unwrap().as_str();
            let expr = pratt_parse(pairs.next().unwrap().into_inner(), pratt);
            Some(Item::DefItem { ident, expr })
        }
        Rule::EOI => None,
        _ => unreachable!(),
    }
}

pub fn parse_file(input: &str) -> Result<Program, Error<Rule>> {
    let pratt = build_pratt_parser();
    let items: Vec<Item> = SirenParser::parse(Rule::program, input)?
        .inspect(|r| println!("{r}"))
        .filter_map(|p| parse_item(p, &pratt))
        .collect();

    Ok(Program(items))
}

pub fn parse_line(input: &str) -> Result<Option<Item>, Error<Rule>> {
    let pratt = build_pratt_parser();
    SirenParser::parse(Rule::item, input).map(|mut pairs| parse_item(pairs.next().unwrap(), &pratt))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let file = r#"let main = 42 + 1;"#;

        let p = parse_file(file).unwrap();
        println!("{:#?}", p);
    }
}
