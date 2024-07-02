use super::prelude::*;

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
            Rule::ident => Expr::Id(Ident(primary.as_str())),
            Rule::number => Expr::Literal(Literal::Number(primary.as_str().parse().unwrap())),
            Rule::boolean => Expr::Literal(Literal::Boolean(primary.as_str().parse().unwrap())),
            // Rule::fn_call => {
            //     let mut pairs = primary.into_inner(); //  ident ~ [ expr ]
            //     let ident = pairs.next().unwrap(); // ident
            //     let func = Box::new(Expr::Id(Ident(ident.as_str())));
            //     let args = pairs.map(|p| pratt_parse(p.into_inner(), pratt)).collect(); // [ expr ]
            //     Expr::Call { func, args }
            // }
            Rule::r#fn => Expr::Fn(parse_function_def(primary.into_inner(), pratt)),
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
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::call => {
                let pairs = op.into_inner(); // [ expr ]
                let args = pairs
                    .map(|expr| pratt_parse(expr.into_inner(), pratt))
                    .collect();
                Expr::Call {
                    func: Box::new(lhs),
                    args,
                }
            }
            _ => unreachable!("get unexpected postfix operator in pratt: {op:?}"),
        })
        .parse(expr)
}
