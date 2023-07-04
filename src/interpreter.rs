use std::{collections::HashMap, fmt::Display};

use crate::parser::{self, *};

enum Value {
    Int(i64),
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
                    .map(|stmt| format!("{}; ", stmt))
                    .collect::<String>()
            ),
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
    pub fn eval(&mut self, ast: Statement) -> Result<Option<i64>, String> {
        match ast {
            Statement::Bind(bind) => {
                let name = bind.name.clone();
                let value = self.eval_expr(bind.value);
                match value {
                    Ok(v) => {
                        self.bind(&name, v)?;
                        Ok(None)
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            Statement::Expr(expr) => Ok(Some(self.eval_expr(expr)?)),
            Statement::Set(set) => {
                let value = self.eval_expr(set.value);
                match value {
                    Ok(v) => {
                        self.set(&set.name, v)?;
                        Ok(None)
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
        }
    }

    // evaluate expression
    fn eval_expr(&self, expr: Expr) -> Result<Value, &str> {
        match expr {
            Expr::Factor(f) => match f {
                parser::Value::Num(n) => Ok(Value::Int(n)),
                parser::Value::Variable(id) => match self.get(&id) {
                    Some(n) => Ok(*n),
                    None => Err("no such variable"),
                },
                parser::Value::Function(params, body) => todo!(),
            },
            Expr::UnExpr(_, n) => Ok(self.eval_expr(*n).map(|Value::Int(n)| Value::Int(-n))?),
            Expr::BinExpr(l, op, r) => match op {
                // todo : type check should be considered here!
                // I need to complete type system for this!
                Infix::Add => Ok(self.eval_expr(*l)? + self.eval_expr(*r)?),
                Infix::Sub => Ok(self.eval_expr(*l)? - self.eval_expr(*r)?),
                Infix::Mul => Ok(self.eval_expr(*l)? * self.eval_expr(*r)?),
                Infix::Div => Ok(self.eval_expr(*l)? / self.eval_expr(*r)?),
            },
        }
    }

    // bind variable to the variable table
    fn bind(&mut self, name: &str, value: i64) -> Result<(), &str> {
        if self.variables.contains_key(name) {
            return Err("Variable already exists");
        }
        self.variables.insert(name.to_string(), Value::Int(value));
        Ok(())
    }

    // set(update) variable in the variable table
    fn set(&mut self, name: &str, value: i64) -> Result<(), &str> {
        if !self.variables.contains_key(name) {
            return Err("Variable not exists");
        }
        self.variables.insert(name.to_string(), Value::Int(value));
        Ok(())
    }
    fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name).as_deref()
    }
}
