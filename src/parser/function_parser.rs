use pest::iterators::Pairs;

use super::*;

pub fn parse_function_def<'a>(
    mut pairs: Pairs<'a, Rule>,
    pratt: &PrattParser<Rule>,
) -> Function<'a> {
    let params = pairs
        .next()
        .unwrap()
        .into_inner() // [ ident ]
        .map(|arg| arg.as_str())
        .collect();

    let body = pairs
        .next()
        .unwrap()
        .into_inner()
        .map(|line| match line.as_rule() {
            Rule::let_stmt => {
                let mut pairs = line.into_inner(); // let_stmt > ident ~ expr
                let ident = pairs.next().unwrap().as_str();
                let expr = pratt_parse(pairs.next().unwrap().into_inner(), pratt);
                Statement::Let(Ident(ident), Box::new(expr))
            }
            Rule::set_stmt => {
                let mut pairs = line.into_inner(); // set_stmt > ident ~ expr
                let ident = pairs.next().unwrap().as_str();
                let expr = pratt_parse(pairs.next().unwrap().into_inner(), pratt);
                Statement::Set(Ident(ident), Box::new(expr))
            }
            Rule::ret_stmt => Statement::Return(Box::new(pratt_parse(
                line.into_inner().next().unwrap().into_inner(), // ret_stmt > expr
                pratt,
            ))),
            Rule::expr => Statement::Return(Box::new(pratt_parse(line.into_inner(), pratt))), // expr
            p => unreachable!("get unexpected statement in function define: {p:?}"),
        })
        .collect();
    Function { params, body }
}
