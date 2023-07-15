mod value;

use std::{collections::HashMap, fmt::Display, iter::zip};

use crate::parser::*;
pub use value::Value;

macro_rules! get_value {
    ($value: expr, $type: ident) => {
        match $value {
            Value::$type(value) => Ok(value),
            _ => Err(format!(
                "Not matched type: expect {}, found {}.",
                stringify!($type),
                $value
            )),
        }
    };
}

#[derive(Default, Clone)]
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
                let value = self.eval_expr(value)?;
                self.bind(&name, value)?;
                Ok(None)
            }
            Statement::Expr(expr) => Ok(Some(self.eval_expr(&expr)?)),
            Statement::Set { name, value } => {
                let value = self.eval_expr(&value)?;
                self.set(&name, value)?;
                Ok(None)
            }
            Statement::Return(ret) => Ok(Some(self.eval_expr(&ret)?)),
        }
    }

    // evaluate expression
    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Ident(ident) => match self.get(ident) {
                Some(n) => Ok(n.clone()),
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
                Infix::Eql => eval_eql(self.eval_expr(l)?, self.eval_expr(r)?),
                Infix::Neq => eval_neq(self.eval_expr(l)?, self.eval_expr(r)?),
                Infix::Lt => todo!(),
                Infix::Gt => todo!(),
                Infix::Lte => todo!(),
                Infix::Gte => todo!(),
                Infix::Call => unreachable!(),
                Infix::Index => unreachable!(),
            },
            Expr::Call { func, args } => {
                let args = args
                    .iter()
                    .map(|arg| self.eval_expr(arg))
                    .collect::<Result<Vec<Value>, String>>()?;
                match func.as_ref() {
                    Expr::Ident(func_name) => {
                        let func = self.get(func_name).ok_or("no such function")?;
                        let (params, body) = match func {
                            Value::Fn { params, body } => Ok((params, body)),
                            _ => Err("this is not a function".to_string()),
                        }?;

                        eval_func(params, args, body, Some(func_name))
                    }
                    Expr::Function { params, body } => eval_func(params, args, body, None),
                    _ => Err("Calling non-function".to_string()),
                }
            }
            Expr::Index { .. } => todo!(),
            Expr::If { cond, then, els } => {
                let cond = self.eval_expr(cond)?;
                eval_if(self, cond, then, els)
            }
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

fn eval_eql(left: Value, right: Value) -> Result<Value, String> {
    match left {
        Value::Int(n) => Ok(Value::Bool(get_value!(right, Int)? == n)),
        Value::Bool(b) => Ok(Value::Bool(get_value!(right, Bool)? == b)),
        Value::Fn { .. } => Err("Cannot compare function".to_string()),
        Value::Unit => match right {
            Value::Unit => Ok(Value::Bool(true)),
            _ => Err(format!(
                "Not matched type: expect {}, found {}.",
                stringify!(Value::Unit),
                right
            )),
        },
    }
}
fn eval_neq(left: Value, right: Value) -> Result<Value, String> {
    match left {
        Value::Int(n) => Ok(Value::Bool(get_value!(right, Int)? != n)),
        Value::Bool(b) => Ok(Value::Bool(get_value!(right, Bool)? != b)),
        Value::Fn { .. } => Err("Cannot compare function".to_string()),
        Value::Unit => match right {
            Value::Unit => Ok(Value::Bool(false)),
            _ => Err(format!(
                "Not matched type: expect {}, found {}.",
                stringify!(Value::Unit),
                right
            )),
        },
    }
}

fn eval_block(mut env: Environment, block: &Vec<Statement>) -> Result<Value, String> {
    let mut result: Option<Value> = None;
    for stmt in block {
        if let Some(r) = env.eval(stmt.clone())? {
            result = Some(r);
        }
    }
    match result {
        Some(value) => Ok(value),
        None => Ok(Value::Unit),
    }
}

fn eval_if(
    env: &Environment,
    cond: Value,
    then: &BlockStmt,
    els: &Option<BlockStmt>,
) -> Result<Value, String> {
    if get_value!(cond, Bool)? {
        return eval_block(env.clone(), then);
    } else if let Some(block) = els {
        return eval_block(env.clone(), block);
    }

    todo!()
}

fn eval_func(
    params: &Vec<String>,
    args: Vec<Value>,
    body: &Vec<Statement>,
    self_func: Option<&str>,
) -> Result<Value, String> {
    let mut env = Environment::new();
    if let Some(name) = self_func {
        env.bind(
            name,
            Value::Fn {
                params: params.to_vec(),
                body: body.to_vec(),
            },
        )?;
    }
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
