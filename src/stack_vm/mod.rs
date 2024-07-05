mod stack;
mod value;

mod prelude {
    pub use super::stack::*;
    pub use super::value::*;

    pub use super::super::*;
}

use prelude::*;

pub use stack::VM;
pub use value::Value;

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
            Ok(Value::Number(v)) => assert_eq!(v, -1.2),
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
            Ok(Value::Number(v)) => assert_eq!(v, -0.82142866),
            Err(_) => todo!(),
        }
    }
}
