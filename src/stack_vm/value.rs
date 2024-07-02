use std::fmt::Display;

use super::prelude::*;

#[derive(Clone)]
pub enum Value {
    Number(f32),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}

pub type Pointer = usize;
