use std::fmt::Display;

pub enum RuntimeError {
    TypeMismatch(String),
    StackUnderFlow,
    BadInstruction(String),
    UndefinedVariable(String),
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            RuntimeError::StackUnderFlow => write!(f, "Stack is empty"),
            RuntimeError::BadInstruction(msg) => write!(f, "Instruction Invalid: {}", msg),
            RuntimeError::UndefinedVariable(msg) => write!(f, "Undefined variable: {}", msg),
        }
    }
}
