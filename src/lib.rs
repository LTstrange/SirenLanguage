mod expr;

use expr::*;
pub struct Parser;

impl Parser {
    pub fn parse(_input: &str) -> Result<Expr, String> {
        match expr(_input) {
            Ok((_, e)) => Ok(e),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
