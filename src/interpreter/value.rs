use std::fmt::Display;

use crate::parser::Statement;

#[derive(Clone)]
pub enum Value {
    Unit,
    Int(i64),
    Bool(bool),
    Fn {
        params: Vec<String>,
        body: Vec<Statement>,
    },
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(num) => write!(f, "{}", num),
            Value::Fn { params, body } => write!(
                f,
                "fn ({}) {{ {}}}",
                params.join(", "),
                body.iter()
                    .map(|stmt| format!("{:?}; ", stmt))
                    .collect::<String>()
            ),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Unit => write!(f, "()"),
        }
    }
}
