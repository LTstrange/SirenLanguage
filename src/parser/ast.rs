use std::fmt::{self, Debug, Formatter};

pub struct Program(pub BlockStmt);

pub type BlockStmt = Vec<Statement>;

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
    Function {
        params: Vec<String>,
        body: BlockStmt,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        arr: Box<Expr>,
        index: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then: BlockStmt,
        els: Option<BlockStmt>,
    },
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
                "{:?}({})",
                func,
                args.iter()
                    .map(|expr| format!("{:?}", expr))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Expr::Index { .. } => todo!(),
            Expr::If { cond, then, els } => match els {
                Some(els_block) => write!(
                    f,
                    "if {:?} {{ {}}} else {{ {}}}",
                    cond,
                    then.iter()
                        .map(|stmt| format!("{:?}; ", stmt))
                        .collect::<String>(),
                    els_block
                        .iter()
                        .map(|stmt| format!("{:?}; ", stmt))
                        .collect::<String>()
                ),
                None => write!(
                    f,
                    "if {:?} {{ {}}}",
                    cond,
                    then.iter()
                        .map(|stmt| format!("{:?}; ", stmt))
                        .collect::<String>()
                ),
            },
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
            Infix::Call => unreachable!(),
            Infix::Index => unreachable!(),
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
