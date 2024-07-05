mod chunk;
mod instruction;

mod prelude {
    pub use super::chunk::*;
    pub use super::instruction::*;

    pub use super::super::*;
}

use prelude::*;

pub use chunk::Chunk;
pub use instruction::Inst;

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
                Infix::Add => {
                    chunk.add_inst(Inst::Add);
                }
                Infix::Sub => {
                    chunk.add_inst(Inst::Sub);
                }
                Infix::Mul => {
                    chunk.add_inst(Inst::Mul);
                }
                Infix::Div => {
                    chunk.add_inst(Inst::Div);
                }
            }
        }
        Expr::Prefix(op, rhs) => {
            compile_expr(*rhs, chunk)?;
            match op {
                Prefix::Neg => {
                    chunk.add_inst(Inst::Neg);
                }
            }
        }
        Expr::Fn(_) => {
            todo!()
        }
        Expr::Call { .. } => {
            todo!()
        }
    }
    Ok(())
}

pub fn compile_ast(program: Program) -> Result<Chunk, String> {
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
pub fn compile(program: Program) -> Result<Chunk, String> {
    todo!()
}
