use std::fmt::Display;

#[derive(Debug)]
pub enum ParserError {
    Default(String),
}
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::Default(s) => write!(f, "{}", s),
        }
    }
}
