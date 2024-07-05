mod chunk;
mod instruction;

use super::*;

pub use chunk::{disassemble_chunk, Chunk};
pub use instruction::Inst;

fn compile_expr(expr: Expr, chunk: &mut Chunk) -> Result<(), String> {
    match expr {
        Expr::Id(name) => {
            let ind = chunk.add_constant(Value::String(name.to_string()));
            chunk.add_inst(Inst::GetGlobal(ind));
        }
        Expr::Literal(Literal::Boolean(_b)) => {
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

fn compile_item(item: Item, chunk: &mut Chunk) -> Result<(), String> {
    match item {
        Item::DefItem { ident, expr } => {
            compile_expr(expr, chunk)?;
            let ind = chunk.add_constant(Value::String(ident.to_string()));
            chunk.add_inst(Inst::DefineGlobal(ind));
        }
    };
    Ok(())
}
pub fn compile(program: Program) -> Result<Chunk, String> {
    let mut chunk = Chunk::new();
    for item in program.0 {
        compile_item(item, &mut chunk)?;
    }
    Ok(chunk)
}
