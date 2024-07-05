mod instruction;
mod stack;
mod value;

mod prelude {
    pub use super::instruction::*;
    pub use super::stack::*;
    pub use super::value::*;
}

use super::*;
use prelude::*;

pub use stack::VM;

fn compile_expr(expr: Expr, chunk: &mut Chunk) -> Result<(), String> {
    match expr {
        Expr::Id(name) => todo!(),
        Expr::Literal(Literal::Boolean(b)) => {
            // let b  = chunk.add_constant(Value::(b));
            // chunk.add_inst(Inst::Const(b));
            todo!()
        }
        Expr::Literal(Literal::Number(n)) => {
            let n = chunk.add_constant(Value::Number(n));
            chunk.add_inst(Inst::Const(n));
        }
        Expr::BinOp(lhs, op, rhs) => {
            compile_expr(*lhs, chunk)?;
            compile_expr(*rhs, chunk)?;
            match op {
                Infix::Add => {chunk.add_inst(Inst::Add); }
                Infix::Sub => {chunk.add_inst(Inst::Sub); }
                Infix::Mul => {chunk.add_inst(Inst::Mul); }
                Infix::Div => {chunk.add_inst(Inst::Div); }
            }
        }
        Expr::Prefix(op, rhs) => {
            compile_expr(*rhs, chunk)?;
            match op {
                Prefix::Neg => {chunk.add_inst(Inst::Neg); }
            }
        }
        Expr::Fn(_) => {todo!()}
        Expr::Call { .. } => {todo!()}
    }
    Ok(())
}

pub fn compile(program: Program) -> Result<Chunk, String> {
    todo!()
}

pub fn compile_item(item: Item) -> Result<Chunk, String> {
    let mut chunk = Chunk::new();
    match item {
        Item::Expr(expr) => compile_expr(*expr, &mut chunk)?,
        Item::DefItem { ident, expr } => todo!(),
    };
    Ok(chunk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_run() {
        let mut chunk = Chunk::new();
        let c = chunk.add_constant(Value::Number(1.2));
        chunk.add_inst(Inst::Const(c));
        chunk.add_inst(Inst::Neg);
        chunk.add_inst(Inst::Ret);
        let mut vm = VM::new(&chunk);

        match vm.run() {
            Ok(v) => println!("{}", v),
            Err(_) => todo!(),
        }
    }

    #[test]
    fn test_simple_expression() {
        let mut chunk = Chunk::new();
        let a = chunk.add_constant(Value::Number(1.2));
        let b = chunk.add_constant(Value::Number(3.4));
        let c = chunk.add_constant(Value::Number(5.6));
        chunk.add_inst(Inst::Const(a));
        chunk.add_inst(Inst::Const(b));
        chunk.add_inst(Inst::Add);
        chunk.add_inst(Inst::Const(c));
        chunk.add_inst(Inst::Div);
        chunk.add_inst(Inst::Neg);
        chunk.add_inst(Inst::Ret);
        let mut vm = VM::new(&chunk);
        match vm.run() {
            Ok(v) => println!("{}", v),
            Err(_) => todo!(),
        }
    }
}
