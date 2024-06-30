#[derive(Debug)]
pub struct Program<'a>(pub Vec<Item<'a>>);

#[derive(Debug)]
pub enum Item<'a> {
    DefItem { ident: &'a str, expr: Expr<'a> },
}

#[derive(Debug)]
pub enum Expr<'a> {
    Id(Ident<'a>),
    Literal(Literal),
    BinOp(Box<Expr<'a>>, Infix, Box<Expr<'a>>),
    UnaryOp(Prefix, Box<Expr<'a>>),
    Fn(Function<'a>),
    Call {
        func: Box<Expr<'a>>,
        args: Vec<Expr<'a>>,
    },
}

#[derive(Debug)]
pub enum Literal {
    Number(f32),
    Boolean(bool),
}

#[derive(Debug)]
pub enum Infix {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Prefix {
    Neg,
}

#[derive(Debug)]
pub struct Ident<'a>(pub &'a str);

#[derive(Debug)]
pub struct Function<'a> {
    params: Vec<String>,
    body: Vec<Item<'a>>, // todo: change to Statements
}
