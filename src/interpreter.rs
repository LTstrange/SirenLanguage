use std::{collections::HashMap, fmt::Display, iter::zip};

use crate::parser::*;

pub enum Value {
    Int(i64),
    Bool(bool),
    Fn {
        params: Vec<String>,
        body: Vec<Statement>,
    },
}

macro_rules! get_value {
    ($value: expr, $type: ident) => {
        match $value {
            Value::$type(value) => Ok(value),
            _ => Err("Not matched type".to_string()),
        }
    };
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
        }
    }
}

#[derive(Default)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.variables {
            writeln!(f, "{} = {}", key, value)?;
        }
        Ok(())
    }
}

impl Environment {
    pub fn new() -> Self {
        Environment::default()
    }

    // evaluate oneline code
    pub fn eval(&mut self, ast: Statement) -> Result<Option<Value>, String> {
        match ast {
            Statement::Let { name, ref value } => {
                self.bind(&name, self.eval_expr(value)?)?;
                Ok(None)
            }
            Statement::Expr(expr) => Ok(Some(self.eval_expr(&expr)?)),
            Statement::Set { name, value } => {
                self.set(&name, self.eval_expr(&value)?)?;
                Ok(None)
            }
            Statement::Return(ret) => Ok(Some(self.eval_expr(&ret)?)),
        }
    }

    // evaluate expression
    fn eval_expr(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Ident(ident) => match self.get(ident) {
                Some(n) => match n {
                    Value::Int(i) => Ok(Value::Int(*i)),
                    Value::Fn { params, body } => Ok(Value::Fn {
                        params: params.clone(),
                        body: body.clone(),
                    }),
                    Value::Bool(b) => Ok(Value::Bool(*b)),
                },
                None => Err("no such variable".to_string()),
            },
            Expr::Literal(literal) => match literal {
                Literal::Int(n) => Ok(Value::Int(*n)),
                Literal::Bool(b) => Ok(Value::Bool(*b)),
            },
            Expr::Function { params, body } => Ok(Value::Fn {
                params: params.to_owned(),
                body: body.to_owned(),
            }),
            // Expr::UnExpr(_, ref n) => Ok(Value::Int(-get_value!(self.eval_expr(n)?, Int)?)),
            Expr::UnExpr(op, ref n) => {
                let value = self.eval_expr(n)?;
                match op {
                    Prefix::Minus => Ok(Value::Int(-get_value!(value, Int)?)),
                    Prefix::Not => Ok(Value::Bool(!get_value!(value, Bool)?)),
                }
            }
            Expr::BinExpr(ref l, op, ref r) => match op {
                Infix::Add => eval_add(self.eval_expr(l)?, self.eval_expr(r)?),
                Infix::Sub => eval_sub(self.eval_expr(l)?, self.eval_expr(r)?),
                Infix::Mul => eval_mul(self.eval_expr(l)?, self.eval_expr(r)?),
                Infix::Div => eval_div(self.eval_expr(l)?, self.eval_expr(r)?),
                Infix::Eql => todo!(),
            },
            Expr::Call { func, args } => match func.as_ref() {
                Expr::Ident(func_name) => {
                    let func = self.get(func_name).ok_or("no such function")?;
                    let (params, body) = match func {
                        Value::Fn { params, body } => Ok((params, body)),
                        _ => Err("this is not a function".to_string()),
                    }?;

                    let args = args
                        .iter()
                        .map(|arg| self.eval_expr(arg))
                        .collect::<Result<Vec<Value>, String>>()?;
                    eval_func(params, args, body, func_name)
                }
                Expr::Function { params, body } => todo!(),
                _ => Err("Calling non-function".to_string()),
            },
        }
    }

    // bind variable to the variable table
    fn bind(&mut self, name: &str, value: Value) -> Result<(), &str> {
        if self.variables.contains_key(name) {
            return Err("Variable already exists");
        }
        self.variables.insert(name.to_string(), value);
        Ok(())
    }

    // set(update) variable in the variable table
    fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
        if !self.variables.contains_key(name) {
            return Err("Variable not exists".to_string());
        }
        match self.variables.get_mut(name) {
            Some(origin) => *origin = value,
            None => return Err("Variable not exists".to_string()),
        }
        Ok(())
    }
    fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
}

fn eval_add(left: Value, right: Value) -> Result<Value, String> {
    Ok(Value::Int(get_value!(left, Int)? + get_value!(right, Int)?))
}

fn eval_sub(left: Value, right: Value) -> Result<Value, String> {
    Ok(Value::Int(get_value!(left, Int)? - get_value!(right, Int)?))
}

fn eval_mul(left: Value, right: Value) -> Result<Value, String> {
    Ok(Value::Int(get_value!(left, Int)? * get_value!(right, Int)?))
}

fn eval_div(left: Value, right: Value) -> Result<Value, String> {
    Ok(Value::Int(get_value!(left, Int)? / get_value!(right, Int)?))
}

fn eval_func(
    params: &Vec<String>,
    args: Vec<Value>,
    body: &Vec<Statement>,
    self_func: &str,
) -> Result<Value, String> {
    let mut env = Environment::new();
    env.bind(
        self_func,
        Value::Fn {
            params: params.to_vec(),
            body: body.to_vec(),
        },
    )?;
    for (param, arg) in zip(params, args) {
        env.bind(param, arg)?;
    }
    let mut result: Option<Value> = None;
    for stmt in body {
        if let Some(r) = env.eval(stmt.clone())? {
            result = Some(r);
        }
    }
    result.ok_or_else(|| "Function return value is None".to_string())
}
