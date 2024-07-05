use super::*;
use std::ops::Index;

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
        if let Some(ind) = get_const_ind(self, &value) {
            ind
        } else {
            self.constants.push(value);
            (self.constants.len() - 1) as u8
        }
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

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    for (i, inst) in chunk.code.iter().enumerate() {
        println!("{:04} {}", i, inst.disassemble(chunk));
    }
}

pub fn get_const_ind(chunk: &Chunk, value: &Value) -> Option<u8> {
    chunk
        .constants
        .iter()
        .enumerate()
        .find(|(_, stored_value)| stored_value == &value)
        .map(|(ind, _)| ind as u8)
}
