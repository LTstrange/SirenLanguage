pub type Program = Vec<Statement>;

#[derive(Clone)]
pub enum Statement {
    Let { name: String, value: Expr },
    Expr(Expr),
    Set { name: String, value: Expr },
}

#[derive(Clone)]
pub enum Literal {
    Int(i64),
    Bool(bool),
    // Str(String),
}

#[derive(Clone)]
pub enum Expr {
    Ident(String),
    Literal(Literal),
    UnExpr(Prefix, Box<Expr>),
    BinExpr(Box<Expr>, Infix, Box<Expr>),
    Function { params: Vec<String>, body: Program },
}

#[derive(Clone)]
pub enum Prefix {
    Minus,
}

#[derive(Debug, Clone)]
pub enum Infix {
    Add,
    Sub,
    Mul,
    Div,
}

// impl Display for Expr {
//     fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
//         use self::Expr::*;
//         match self {
//             Factor(val) => match val {
//                 Value::Num(n) => write!(format, "{}", n),
//                 Value::Variable(v) => write!(format, "{}", v),
//                 Value::Function(args, _stmts) => {
//                     write!(
//                         format,
//                         "fn ({}) {{ {}}}",
//                         args.join(", "),
//                         _stmts
//                             .iter()
//                             .map(|stmt| format!("{}; ", stmt))
//                             .collect::<String>()
//                     )
//                 }
//             },
//             UnExpr(op, right) => match op {
//                 Prefix::Minus => write!(format, "(-{})", right),
//             },
//             BinExpr(left, op, right) => match op {
//                 Infix::Add => write!(format, "({} + {})", left, right),
//                 Infix::Sub => write!(format, "({} - {})", left, right),
//                 Infix::Mul => write!(format, "({} * {})", left, right),
//                 Infix::Div => write!(format, "({} / {})", left, right),
//             },
//         }
//     }
// }

// impl Display for Statement {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Statement::Let(bind) => write!(f, "Bind: {}", bind),
//             Statement::Expr(expr) => write!(f, "Expr: {}", expr),
//             Statement::Set(set) => write!(f, "Set: {}", set),
//         }
//     }
// }

// impl Display for Let {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "let {} = {}", self.name, self.value)
//     }
// }

// set value to the coresponding variable : assignment

// impl Display for Set {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} = {}", self.name, self.value)
//     }
// }
