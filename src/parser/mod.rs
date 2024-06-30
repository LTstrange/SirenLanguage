mod ast;
mod function_parser;
mod pratt_parser;

use ast::*;
use pratt_parser::*;

use pest::{error::Error, iterators::Pair, pratt_parser::PrattParser, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct SirenParser;

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

    fn test_input(input: &str) {
        match parse_file(input) {
            Ok(p) => println!("{:#?}", p),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_calculate_expression() {
        let input = r#"let a = 123 - 12 / 4; let b = (-a + 42) / 2;"#;
        test_input(input);
    }

    #[test]
    fn test_call() {
        let input = r#"let main = sum(a, 13);"#;
        test_input(input);
    }

    #[test]
    fn test_func_build() {
        let input = r#"
let main = fn(a, b){
    let a = b;
    a = b;
    return a;
    42
};"#;
        test_input(input);
    }
}
