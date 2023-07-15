use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::parser::Statement;

macro_rules! get_value_typename {
    ($value: ident) => {
        match $value {
            Value::Unit => "Unit",
            Value::Int(_) => "Int",
            Value::Bool(_) => "Bool",
            Value::Fn { .. } => "Fn",
        }
    };
}

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

pub fn calc_eql(left: Value, right: Value) -> Result<Value, String> {
    match (&left, &right) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
        (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
        (Value::Fn { .. }, _) => Err("Cannot compare function".to_string()),
        (Value::Unit, Value::Unit) => Ok(Value::Bool(false)),
        _ => Err(format!(
            "Not matched type: left: {}, right: {}.",
            get_value_typename!(left),
            get_value_typename!(right)
        )),
    }
}
pub fn calc_neq(left: Value, right: Value) -> Result<Value, String> {
    match (&left, &right) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
        (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
        (Value::Fn { .. }, _) => Err("Cannot compare function".to_string()),
        (Value::Unit, Value::Unit) => Ok(Value::Bool(false)),
        _ => Err(format!(
            "Not matched type: left: {}, right: {}.",
            get_value_typename!(left),
            get_value_typename!(right)
        )),
    }
}

pub fn calc_lt(left: Value, right: Value) -> Result<Value, String> {
    match (&left, &right) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
        (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a < b)),
        (Value::Fn { .. }, _) => Err("Cannot compare function".to_string()),
        (Value::Unit, Value::Unit) => Ok(Value::Bool(false)),
        _ => Err(format!(
            "Not matched type: left: {}, right: {}.",
            get_value_typename!(left),
            get_value_typename!(right)
        )),
    }
}

pub fn calc_lte(left: Value, right: Value) -> Result<Value, String> {
    match (&left, &right) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
        (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a <= b)),
        (Value::Fn { .. }, _) => Err("Cannot compare function".to_string()),
        (Value::Unit, Value::Unit) => Ok(Value::Bool(true)),
        _ => Err(format!(
            "Not matched type: left: {}, right: {}.",
            get_value_typename!(left),
            get_value_typename!(right)
        )),
    }
}

pub fn calc_gt(left: Value, right: Value) -> Result<Value, String> {
    match (&left, &right) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
        (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a > b)),
        (Value::Fn { .. }, _) => Err("Cannot compare function".to_string()),
        (Value::Unit, Value::Unit) => Ok(Value::Bool(false)),
        _ => Err(format!(
            "Not matched type: left: {}, right: {}.",
            get_value_typename!(left),
            get_value_typename!(right)
        )),
    }
}

pub fn calc_gte(left: Value, right: Value) -> Result<Value, String> {
    match (&left, &right) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
        (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a >= b)),
        (Value::Fn { .. }, _) => Err("Cannot compare function".to_string()),
        (Value::Unit, Value::Unit) => Ok(Value::Bool(true)),
        _ => Err(format!(
            "Not matched type: left: {}, right: {}.",
            get_value_typename!(left),
            get_value_typename!(right)
        )),
    }
}
