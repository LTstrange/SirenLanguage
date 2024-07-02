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

pub fn compile(program: Program) -> Result<Chunk, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
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
}
