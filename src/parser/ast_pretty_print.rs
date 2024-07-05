use super::*;

macro_rules! print_with_tab {
    ($depth: expr, $content: expr) => {
        println!("{}{}", "|  ".repeat($depth as usize), $content)
    };
}

/// For pretty-printing AST nodes.

pub fn pretty_print_program(program: &Program, depth: u8) {
    print_with_tab!(depth, "(Program");
    for item in &program.0 {
        pretty_print_item(item, depth + 1);
    }
    print_with_tab!(depth, ")");
}

fn pretty_print_item(item: &Item, depth: u8) {
    match item {
        Item::DefItem {
            ident: Ident(name),
            expr,
        } => {
            print_with_tab!(depth, format!("(bind {}", name));
            pretty_print_expr(expr, depth + 1);
            print_with_tab!(depth, ")");
        }
    }
}

fn pretty_print_expr(expr: &Expr, depth: u8) {
    match expr {
        Expr::Id(ident) => print_with_tab!(depth, ident),
        Expr::Literal(Literal::Boolean(b)) => print_with_tab!(depth, b),
        Expr::Literal(Literal::Number(n)) => print_with_tab!(depth, n),
        Expr::Literal(Literal::String(s)) => print_with_tab!(depth, format!("{:?}", s)),
        Expr::BinOp(lhs, op, rhs) => {
            print_with_tab!(depth, format!("({}", op));
            pretty_print_expr(lhs, depth + 1);
            pretty_print_expr(rhs, depth + 1);
            print_with_tab!(depth, ")");
        }
        Expr::Prefix(op, rhs) => {
            print_with_tab!(depth, format!("({}", op));
            pretty_print_expr(rhs, depth + 1);
            print_with_tab!(depth, ")");
        }
        Expr::Fn(Function { params, body }) => {
            print_with_tab!(depth, "(fn");
            pretty_print_params(params, depth + 1);
            pretty_print_block(body, depth + 1);
            print_with_tab!(depth, ")");
        }
        Expr::Call { func, args } => {
            print_with_tab!(depth, "(call");
            pretty_print_expr(func, depth + 1);
            pretty_print_args(args, depth + 1);
            print_with_tab!(depth, ")");
        }
    }
}

fn pretty_print_args(args: &[Expr], depth: u8) {
    print_with_tab!(depth, "(args");
    for arg in args {
        pretty_print_expr(arg, depth + 1);
    }
    print_with_tab!(depth, ")");
}

fn pretty_print_params(params: &[Ident], depth: u8) {
    let ps = params
        .iter()
        .fold(String::new(), |acc, p| acc + &format!(" {}", p));
    print_with_tab!(depth, format!("(params{})", ps));
}
fn pretty_print_block(body: &[Statement], depth: u8) {
    print_with_tab!(depth, "(body");
    for statement in body {
        pretty_print_statement(statement, depth + 1);
    }
    print_with_tab!(depth, ")");
}

fn pretty_print_statement(statement: &Statement, depth: u8) {
    match statement {
        Statement::Let(ident, expr) => {
            print_with_tab!(depth, format!("(bind {}", ident));
            pretty_print_expr(expr, depth + 1);
            print_with_tab!(depth, ")");
        }
        Statement::Return(expr) => {
            print_with_tab!(depth, "(return");
            pretty_print_expr(expr, depth + 1);
            print_with_tab!(depth, ")");
        }
        Statement::Set(ident, expr) => {
            print_with_tab!(depth, format!("(set {}", ident));
            pretty_print_expr(expr, depth + 1);
            print_with_tab!(depth, ")");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_print() {
        let p = parse_file(r#"let main = fn(a,b,c){a = 42;};"#).unwrap();
        pretty_print_program(&p, 0);
    }
}
