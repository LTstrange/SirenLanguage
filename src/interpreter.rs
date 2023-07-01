use std::collections::HashMap;

use crate::parser::{expr::Expr, statement::Statement};

#[derive(Default)]
pub struct Environment {
    variables: HashMap<String, i64>,
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
    fn eval_expr(&self, expr: Expr) -> Result<i64, &str> {
        match expr {
            Expr::Factor(f) => match f {
                crate::parser::expr::Value::Value(n) => Ok(n),
                crate::parser::expr::Value::Variable(id) => match self.get(&id) {
                    Some(n) => Ok(n),
                    None => Err("no such variable"),
                },
            },
            Expr::UnExpr(_, n) => Ok(-self.eval_expr(*n)?),
            Expr::BinExpr(l, op, r) => match op {
                crate::parser::expr::Infix::Add => Ok(self.eval_expr(*l)? + self.eval_expr(*r)?),
                crate::parser::expr::Infix::Sub => Ok(self.eval_expr(*l)? - self.eval_expr(*r)?),
                crate::parser::expr::Infix::Mul => Ok(self.eval_expr(*l)? * self.eval_expr(*r)?),
                crate::parser::expr::Infix::Div => Ok(self.eval_expr(*l)? / self.eval_expr(*r)?),
            },
        }
    }

    // bind variable to the variable table
    fn bind(&mut self, name: &str, value: i64) -> Result<(), &str> {
        if self.variables.contains_key(name) {
            return Err("Variable already exists");
        }
        self.variables.insert(name.to_string(), value);
        Ok(())
    }

    // set(update) variable in the variable table
    fn set(&mut self, name: &str, value: i64) -> Result<(), &str> {
        if !self.variables.contains_key(name) {
            return Err("Variable not exists");
        }
        self.variables.insert(name.to_string(), value);
        Ok(())
    }
    fn get(&self, name: &str) -> Option<i64> {
        self.variables.get(name).cloned()
    }
}
