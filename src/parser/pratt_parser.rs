use super::*;

use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
};

pub fn build_pratt_parser() -> PrattParser<Rule> {
    PrattParser::new()
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
        .op(Op::prefix(Rule::neg))
        .op(Op::postfix(Rule::call))
}

pub fn pratt_parse<'a>(expr: Pairs<'a, Rule>, pratt: &PrattParser<Rule>) -> Expr<'a> {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::ident => Expr::Ident(primary.as_str()),
            Rule::number => Expr::Literal(Literal::Number(primary.as_str().parse().unwrap())),
            Rule::boolean => Expr::Literal(Literal::Boolean(primary.as_str().parse().unwrap())),
            Rule::expr => pratt_parse(primary.into_inner(), pratt),
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
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::call => Expr::Call {
                func: Box::new(lhs),
                args: op
                    .into_inner()
                    .map(|arg| pratt_parse(arg.into_inner(), &pratt))
                    .collect(),
            },
            _ => unreachable!("get unexpected postfix operator in pratt: {op:?}"),
        })
        .parse(expr)
}
