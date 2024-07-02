use std::fmt::Display;

use super::prelude::*;

impl Display for Program<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Program")?;
        for item in &self.0 {
            write!(f, " {}", item)?;
        }
        write!(f, ")")
    }
}

impl Display for Item<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::DefItem { ident, expr } => write!(f, "(bind {} {})", ident, expr),
        }
    }
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Id(Ident(name)) => write!(f, "{}", name),
            Expr::Literal(Literal::Boolean(b)) => write!(f, "{}", b),
            Expr::Literal(Literal::Number(n)) => write!(f, "{}", n),
            Expr::BinOp(lhs, op, rhs) => write!(f, "({} {} {})", op, lhs, rhs),
            Expr::UnaryOp(op, rhs) => write!(f, "({} {})", op, rhs),
            Expr::Fn(function) => write!(f, "{}", function),
            Expr::Call { func, args } => write!(f, "(call {} (args{}))", func, display_args(args)),
        }
    }
}

fn display_args(args: &[Expr]) -> String {
    args.iter().fold(String::new(), |mut acc, arg| {
        acc.push_str(&format!(" {}", arg));
        acc
    })
}

impl Display for Function<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(fn ")?;
        // params
        write!(f, "(params")?;
        for p in self.params.iter() {
            write!(f, " {}", p)?;
        }
        write!(f, ")")?;
        // body
        write!(f, "(body")?;
        for st in self.body.iter() {
            write!(f, " {}", st)?;
        }
        write!(f, "))")
    }
}

impl Display for Statement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(Ident(name), value) => write!(f, "(bind {} {})", name, value),
            Statement::Set(Ident(name), value) => write!(f, "(set {} {})", name, value),
            Statement::Return(value) => write!(f, "(return {})", value),
        }
    }
}

impl Display for Infix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Infix::Add => write!(f, "+"),
            Infix::Sub => write!(f, "-"),
            Infix::Mul => write!(f, "*"),
            Infix::Div => write!(f, "/"),
        }
    }
}

impl Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prefix::Neg => write!(f, "-"),
        }
    }
}

impl Display for Ident<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_file;

    use super::*;

    #[test]
    fn test_program() {
        let a = parse_file(r#""#).unwrap();
        assert_eq!(format!("{a}"), "(Program)");
        let a = parse_file(r#"let main = 42;"#).unwrap();
        assert_eq!(format!("{a}"), "(Program (bind main 42))");
        let a = parse_file(r#"let a = 12; let main = 42;"#).unwrap();
        assert_eq!(format!("{a}"), "(Program (bind a 12) (bind main 42))");
    }
}
