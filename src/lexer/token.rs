use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \n\r\t]+")]
pub enum Token {
    // identifier and literal
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| String::from(lex.slice()))]
    Ident(String),
    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Int(i64),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().ok())]
    Float(f64),
    #[regex(r"true|false", |lex| lex.slice().parse().ok())]
    Bool(bool),

    // type parameters
    #[regex(r"int|bool", |lex| String::from(lex.slice()))]
    TypeParam(String),

    // reserved keywords
    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("return")]
    Return,
    #[token("if")]
    If,
    #[token("else")]
    Else,

    // assignment
    #[token("=")]
    Assign,

    // operators
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,

    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,

    // punctuations
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_keywords() {
        let mut lex = Token::lexer(
            "let fn return if else = + - * / && || ! == != < > <= >= . , : ; ( ) { } [ ]",
        );
        assert_eq!(Some(Ok(Token::Let)), lex.next());
        assert_eq!(Some(Ok(Token::Fn)), lex.next());
        assert_eq!(Some(Ok(Token::Return)), lex.next());
        assert_eq!(Some(Ok(Token::If)), lex.next());
        assert_eq!(Some(Ok(Token::Else)), lex.next());
        assert_eq!(Some(Ok(Token::Assign)), lex.next());
        assert_eq!(Some(Ok(Token::Add)), lex.next());
        assert_eq!(Some(Ok(Token::Sub)), lex.next());
        assert_eq!(Some(Ok(Token::Mul)), lex.next());
        assert_eq!(Some(Ok(Token::Div)), lex.next());

        assert_eq!(Some(Ok(Token::And)), lex.next());
        assert_eq!(Some(Ok(Token::Or)), lex.next());
        assert_eq!(Some(Ok(Token::Not)), lex.next());
        assert_eq!(Some(Ok(Token::Equal)), lex.next());
        assert_eq!(Some(Ok(Token::NotEqual)), lex.next());
        assert_eq!(Some(Ok(Token::Less)), lex.next());
        assert_eq!(Some(Ok(Token::Greater)), lex.next());
        assert_eq!(Some(Ok(Token::LessEqual)), lex.next());
        assert_eq!(Some(Ok(Token::GreaterEqual)), lex.next());
        assert_eq!(Some(Ok(Token::Dot)), lex.next());
        assert_eq!(Some(Ok(Token::Comma)), lex.next());
        assert_eq!(Some(Ok(Token::Colon)), lex.next());
        assert_eq!(Some(Ok(Token::Semicolon)), lex.next());
        assert_eq!(Some(Ok(Token::LParen)), lex.next());
        assert_eq!(Some(Ok(Token::RParen)), lex.next());
        assert_eq!(Some(Ok(Token::LBrace)), lex.next());
        assert_eq!(Some(Ok(Token::RBrace)), lex.next());
        assert_eq!(Some(Ok(Token::LBracket)), lex.next());
        assert_eq!(Some(Ok(Token::RBracket)), lex.next());
    }

    #[test]
    fn test_literals() {
        let mut lex = Token::lexer("123 123.123 true false");
        assert_eq!(Some(Ok(Token::Int(123))), lex.next());
        assert_eq!(Some(Ok(Token::Float(123.123))), lex.next());
        assert_eq!(Some(Ok(Token::Bool(true))), lex.next());
        assert_eq!(Some(Ok(Token::Bool(false))), lex.next());
    }

    #[test]
    fn test_identifier() {
        let mut lex = Token::lexer("hello world _abc");
        assert_eq!(Some(Ok(Token::Ident("hello".to_string()))), lex.next());
        assert_eq!(Some(Ok(Token::Ident("world".to_string()))), lex.next());
        assert_eq!(Some(Ok(Token::Ident("_abc".to_string()))), lex.next());
    }

    #[test]
    fn test_type_parsing() {
        let mut lex = Token::lexer("let abc : int = 123;");
        assert_eq!(Some(Ok(Token::Let)), lex.next());
        assert_eq!(Some(Ok(Token::Ident("abc".to_string()))), lex.next());
        assert_eq!(Some(Ok(Token::Colon)), lex.next());
        assert_eq!(Some(Ok(Token::TypeParam("int".to_string()))), lex.next());
        assert_eq!(Some(Ok(Token::Assign)), lex.next());
        assert_eq!(Some(Ok(Token::Int(123))), lex.next());
        assert_eq!(Some(Ok(Token::Semicolon)), lex.next());

        let mut lex = Token::lexer("let abc : bool = true;");
        assert_eq!(Some(Ok(Token::Let)), lex.next());
        assert_eq!(Some(Ok(Token::Ident("abc".to_string()))), lex.next());
        assert_eq!(Some(Ok(Token::Colon)), lex.next());
        assert_eq!(Some(Ok(Token::TypeParam("bool".to_string()))), lex.next());
        assert_eq!(Some(Ok(Token::Assign)), lex.next());
        assert_eq!(Some(Ok(Token::Bool(true))), lex.next());
        assert_eq!(Some(Ok(Token::Semicolon)), lex.next());
    }
}
