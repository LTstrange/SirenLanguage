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
}

pub fn pratt_parse<'a>(expr: Pairs<'a, Rule>, pratt: &PrattParser<Rule>) -> Expr<'a> {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::ident => Expr::Id(Ident(primary.as_str())),
            Rule::number => Expr::Literal(Literal::Number(primary.as_str().parse().unwrap())),
            Rule::boolean => Expr::Literal(Literal::Boolean(primary.as_str().parse().unwrap())),
            Rule::fn_call => {
                println!("primary: {}", primary);
                let mut pairs = primary.into_inner();
                println!("pairs: {}", pairs);
                let ident = pairs.next().unwrap();
                let func = Box::new(Expr::Id(Ident(ident.as_str())));
                let args = pairs.map(|p| pratt_parse(p.into_inner(), pratt)).collect();
                Expr::Call { func, args }
            }
            Rule::r#fn => {
                let mut pairs = primary.into_inner();
                let params = pairs
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|arg| arg.as_str())
                    .collect();

                let body = pairs
                    .next()
                    .unwrap()
                    .into_inner()
                    .map(|line| match line.as_rule() {
                        Rule::let_stmt => {
                            let mut pairs = line.into_inner(); // ident ~ expr
                            let ident = pairs.next().unwrap().as_str();
                            let expr = pratt_parse(pairs.next().unwrap().into_inner(), pratt);
                            Statement::Let(Ident(ident), Box::new(expr))
                        }
                        Rule::set_stmt => {
                            let mut pairs = line.into_inner(); // ident ~ expr
                            let ident = pairs.next().unwrap().as_str();
                            let expr = pratt_parse(pairs.next().unwrap().into_inner(), pratt);
                            Statement::Set(Ident(ident), Box::new(expr))
                        }
                        Rule::ret_stmt => Statement::Return(Box::new(pratt_parse(
                            line.into_inner().next().unwrap().into_inner(),
                            pratt,
                        ))),
                        Rule::expr => {
                            Statement::Return(Box::new(pratt_parse(line.into_inner(), pratt)))
                        }
                        p => unreachable!("get unexpected statement in pratt: {p:?}"),
                    })
                    .collect();
                Expr::Fn(Function { params, body })
            }
            Rule::expr => pratt_parse(primary.into_inner(), pratt), // "(" ~ expr ~ ")"
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
