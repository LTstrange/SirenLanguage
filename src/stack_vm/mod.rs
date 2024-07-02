use std::fmt::Display;

enum Instruction {
    Ret,
}

struct Chunk {
    instructions: Vec<Instruction>,
    constants: Vec<Value>,
}

#[cfg(debug)]
impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
}
