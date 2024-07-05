use super::*;

pub enum Inst {
    // Push(isize),
    // Pop,
    Add,
    Sub,
    Mul,
    Div,
    // Incr,
    // Decr,
    Neg,
    Const(u8),

    // Jump(Pointer),
    // Je(Pointer),
    // Jne(Pointer),

    // Get(i8),
    // Set(i8),
    DefineGlobal(u8),
    GetGlobal(u8),

    // Call(Pointer),
    Ret,
}

impl Inst {
    pub fn disassemble(&self, chunk: &Chunk) -> String {
        match self {
            Inst::Add => "OP_ADDITION".to_string(),
            Inst::Sub => "OP_SUBTRACT".to_string(),
            Inst::Mul => "OP_MULTIPLY".to_string(),
            Inst::Div => "OP_DIVIDE".to_string(),
            Inst::Neg => "OP_NEGATE".to_string(),
            Inst::Const(ind) => {
                format!(
                    "OP_CONSTANT    {:2}  '{}'",
                    ind,
                    chunk.get_const(*ind as usize)
                )
            }
            Inst::Ret => "OP_RETURN".to_string(),
            Inst::DefineGlobal(ind) => {
                format!(
                    "OP_DEF_GLOBAL  {:2}  '{}'",
                    ind,
                    chunk.get_const(*ind as usize)
                )
            }
            Inst::GetGlobal(ind) => {
                format!(
                    "OP_Get_GLOBAL  {:2}  '{}'",
                    ind,
                    chunk.get_const(*ind as usize)
                )
            }
        }
    }
}
