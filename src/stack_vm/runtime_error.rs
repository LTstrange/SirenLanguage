use std::fmt::Display;

pub enum RuntimeError {
    TypeMismatch(String),
    StackUnderFlow,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            RuntimeError::StackUnderFlow => write!(f, "Stack is empty"),
        }
    }
}
