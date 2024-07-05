mod ast;
mod ast_display;
mod error;
mod function_parser;
mod item_parser;
mod pratt_parser;

mod prelude {
    pub use super::ast::*;
    pub use super::function_parser::*;
    pub use super::item_parser::*;
    pub use super::pratt_parser::*;
    pub use super::Rule;

    pub use pest::{
        iterators::{Pair, Pairs},
        pratt_parser::{Assoc, Op, PrattParser},
        Parser,
    };
    pub use pest_derive::Parser;
}

use prelude::*;

pub use ast::*;
pub use error::*;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct SirenParser;

pub fn parse_file(input: &str) -> Result<Program, ParserError> {
    let pratt = build_pratt_parser();
    let items: Vec<Item> = SirenParser::parse(Rule::program, input)
        .map_err(|e| ParserError::Default(format!("{}", e)))?
        .filter_map(|p| parse_item(p, &pratt))
        .collect();

    Ok(Program(items))
}

pub fn parse_line(input: &str) -> Result<Option<Item>, String> {
    let pratt = build_pratt_parser();
    SirenParser::parse(Rule::repl, input)
        .map(|mut pairs| parse_item(pairs.next().unwrap(), &pratt))
        .map_err(|e| format!("Parse error: {}", e))
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
