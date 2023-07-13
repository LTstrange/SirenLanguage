use std::fmt::{self, Debug, Formatter};

pub type Program = Vec<Statement>;

#[derive(PartialEq, Clone)]
pub enum Statement {
    Let { name: String, value: Expr },
    Expr(Expr),
    Set { name: String, value: Expr },
    Return(Expr),
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Statement::Let { name, value } => write!(f, "let {} = {:?}", name, value),
            Statement::Expr(expr) => write!(f, "expr {:?}", expr),
            Statement::Set { name, value } => write!(f, "set {} = {:?}", name, value),
            Statement::Return(ret) => write!(f, "return {:?}", ret),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Literal {
    Int(i64),
    Bool(bool),
    // Str(String),
}
impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Bool(b) => write!(f, "{}", b),
            // Literal::Str(s) => write!(f, "{}", s),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Expr {
    Ident(String),
    Literal(Literal),
    UnExpr(Prefix, Box<Expr>),
    BinExpr(Box<Expr>, Infix, Box<Expr>),
    Function { params: Vec<String>, body: Program },
    Call { func: Box<Expr>, args: Vec<Expr> },
    Index { arr: Box<Expr>, index: Box<Expr> },
    // If {
    //     cond: Box<Expr>,
    //     then: Box<Expr>,
    //     els: Option<Box<Expr>>,
    // },
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Ident(name) => write!(f, "{}", name),
            Expr::Literal(l) => write!(f, "{:?}", l),
            Expr::UnExpr(op, e) => write!(f, "({:?}{:?})", op, e),
            Expr::BinExpr(l, op, r) => write!(f, "({:?} {:?} {:?})", l, op, r),
            Expr::Function { params, body } => {
                write!(
                    f,
                    "fn({}) {{ {}}}",
                    params.join(", "),
                    body.iter()
                        .map(|stmt| format!("{:?}; ", stmt))
                        .collect::<String>()
                )
            }
            Expr::Call { func, args } => write!(
                f,
                "{:?}.call({})",
                func,
                args.iter()
                    .map(|expr| format!("{:?}", expr))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Expr::Index { arr, index } => todo!(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Prefix {
    Minus,
    Not,
}

impl Debug for Prefix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Prefix::Minus => write!(f, "-"),
            Prefix::Not => write!(f, "!"),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Infix {
    Add,
    Sub,
    Mul,
    Div,
    Eql,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    Call,
    Index,
}

impl Debug for Infix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Infix::Add => write!(f, "+"),
            Infix::Sub => write!(f, "-"),
            Infix::Mul => write!(f, "*"),
            Infix::Div => write!(f, "/"),
            Infix::Eql => write!(f, "=="),
            Infix::Neq => write!(f, "!="),
            Infix::Lt => write!(f, "<"),
            Infix::Gt => write!(f, ">"),
            Infix::Lte => write!(f, "<="),
            Infix::Gte => write!(f, ">="),
            Infix::Call => todo!(),
            Infix::Index => todo!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Prec {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Call,
    Index,
}
