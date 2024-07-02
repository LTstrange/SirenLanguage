use std::ops::Index;

use super::prelude::*;

// chunk of bytecode, and constants
pub struct Chunk {
    code: Vec<Inst>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn get_const(&self, index: usize) -> &Value {
        &self.constants[index]
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }

    pub fn add_inst(&mut self, inst: Inst) {
        self.code.push(inst);
    }
}

impl Index<usize> for Chunk {
    type Output = Inst;

    fn index(&self, index: usize) -> &Self::Output {
        &self.code[index]
    }
}

pub enum Inst {
    // Push(isize),
    // Pop,
    // Add,
    // Mul,
    // Div,
    // Sub,
    // Incr,
    // Decr,
    Neg,
    Const(u8),

    // Jump(Pointer),
    // Je(Pointer),
    // Jne(Pointer),

    // Get(i8),
    // Set(i8),

    // Call(Pointer),
    Ret,
}

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    for (i, inst) in chunk.code.iter().enumerate() {
        print!("{:04} ", i);
        match inst {
            Inst::Const(ind) => {
                println!("OP_CONSTANT\t{} '{}'", ind, chunk.constants[*ind as usize])
            }
            Inst::Ret => println!("OP_RETURN"),
            Inst::Neg => println!("OP_NEGATE"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble_chunk() {
        let chunk = Chunk {
            code: vec![Inst::Ret, Inst::Const(0)],
            constants: vec![Value::Number(42.)],
        };

        disassemble_chunk(&chunk, "test_chunk");
    }
}
