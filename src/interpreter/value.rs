use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

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

impl Add for Value {
    type Output = Result<Value, String>;
    fn add(self, other: Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            _ => Err("Expect int type".to_string()),
        }
    }
}

impl Sub for Value {
    type Output = Result<Value, String>;
    fn sub(self, other: Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            _ => Err("Expect int type".to_string()),
        }
    }
}

impl Mul for Value {
    type Output = Result<Value, String>;
    fn mul(self, other: Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            _ => Err("Expect int type".to_string()),
        }
    }
}

impl Div for Value {
    type Output = Result<Value, String>;
    fn div(self, other: Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
            _ => Err("Expect int type".to_string()),
        }
    }
}
