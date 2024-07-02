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
