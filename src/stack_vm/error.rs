use std::fmt::Display;

pub enum RuntimeError {
    TypeMismatch(String),
    StackEmpty,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            RuntimeError::StackEmpty => write!(f, "Stack is empty"),
        }
    }
}
