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

    // Call(Pointer),
    Ret,
}
