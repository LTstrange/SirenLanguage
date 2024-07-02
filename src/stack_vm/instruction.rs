use super::prelude::*;

// chunk of bytecode, and constants
pub struct Chunk {
    code: Vec<Inst>,
    constants: Vec<Value>,
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
