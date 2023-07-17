mod value;

use std::{cell::RefCell, collections::HashMap, fmt::Display, iter::zip, rc::Rc};

use crate::parser::*;
pub use value::Value;
use value::*;

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

#[derive(Default)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
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
        match self.variables.get_mut(name) {
            Some(origin) => {
                *origin = value;
                Ok(())
            }
            None => match &mut self.parent {
                Some(parent) => parent.borrow_mut().set(name, value),
                None => Err(format!("no such variable: {}", name)),
            },
        }
    }
    fn get(&self, name: &str) -> Option<Value> {
        match self.variables.get(name).cloned() {
            Some(value) => Some(value),
            None => match &self.parent {
                Some(parent) => parent.borrow().get(name),
                None => None,
            },
        }
    }
}

#[derive(Default)]
pub struct Evaluator {
    pub env: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator::default()
    }

    fn init_with_exist(env: Rc<RefCell<Environment>>) -> Self {
        Evaluator { env }
    }

    fn init_with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment {
                parent: Some(parent),
                ..Default::default()
            })),
        }
    }

    // evaluate oneline code
    pub fn eval(&mut self, ast: Statement) -> Result<Option<Value>, String> {
        match ast {
            Statement::Let { name, ref value } => {
                let value = self.eval_expr(value)?;
                self.env.borrow_mut().bind(&name, value)?;
                Ok(None)
            }
            Statement::Expr(expr) => Ok(Some(self.eval_expr(&expr)?)),
            Statement::Set { name, value } => {
                let value = self.eval_expr(&value)?;
                self.env.borrow_mut().set(&name, value)?;
                Ok(None)
            }
            Statement::Return(ret) => Ok(Some(self.eval_expr(&ret)?)),
        }
    }

    // evaluate expression
    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Ident(ident) => match self.env.borrow_mut().get(ident) {
                Some(n) => Ok(n),
                None => Err(format!("no such variable: {}", ident)),
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
                Infix::Add => self.eval_expr(l)? + self.eval_expr(r)?,
                Infix::Sub => self.eval_expr(l)? - self.eval_expr(r)?,
                Infix::Mul => self.eval_expr(l)? * self.eval_expr(r)?,
                Infix::Div => self.eval_expr(l)? / self.eval_expr(r)?,
                Infix::Eql => calc_eql(self.eval_expr(l)?, self.eval_expr(r)?),
                Infix::Neq => calc_neq(self.eval_expr(l)?, self.eval_expr(r)?),
                Infix::Lt => calc_lt(self.eval_expr(l)?, self.eval_expr(r)?),
                Infix::Lte => calc_lte(self.eval_expr(l)?, self.eval_expr(r)?),
                Infix::Gt => calc_gt(self.eval_expr(l)?, self.eval_expr(r)?),
                Infix::Gte => calc_gte(self.eval_expr(l)?, self.eval_expr(r)?),
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
                        let func = self
                            .env
                            .borrow()
                            .get(func_name)
                            .ok_or(format!("no such function: {}", func_name))?;
                        let (params, body) = match func {
                            Value::Fn { params, body } => Ok((params, body)),
                            _ => Err("this is not a function".to_string()),
                        }?;

                        eval_func(&params, args, &body, Some(func_name))
                    }
                    Expr::Function { params, body } => eval_func(params, args, body, None),
                    _ => Err("Calling non-function".to_string()),
                }
            }
            Expr::Index { .. } => todo!(),
            Expr::If { cond, then, els } => {
                let cond = self.eval_expr(cond)?;
                self.eval_if(cond, then, els)
            }
        }
    }

    fn eval_if(
        &mut self,
        cond: Value,
        then: &BlockStmt,
        els: &Option<BlockStmt>,
    ) -> Result<Value, String> {
        if get_value!(cond, Bool)? {
            self.eval_block(then)
        } else if let Some(block) = els {
            self.eval_block(block)
        } else {
            Ok(Value::Unit) // todo : type check system
        }
    }

    fn eval_block(&mut self, block: &BlockStmt) -> Result<Value, String> {
        let mut inner_evaluator = Self::init_with_parent(self.env.clone());
        let mut result: Option<Value> = None;
        for stmt in block {
            if let Some(r) = inner_evaluator.eval(stmt.clone())? {
                result = Some(r);
            }
        }
        match result {
            Some(value) => Ok(value),
            None => Ok(Value::Unit),
        }
    }
}

fn eval_func(
    params: &Vec<String>,
    args: Vec<Value>,
    body: &Vec<Statement>,
    self_func: Option<&str>,
) -> Result<Value, String> {
    let mut env = Environment::default();
    if args.len() != params.len() {
        return Err(format!(
            "expect {} parameters, found {}",
            params.len(),
            args.len()
        ));
    }
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
    let mut evaluator = Evaluator::init_with_exist(Rc::new(RefCell::new(env)));

    let mut result: Option<Value> = None;
    for stmt in body {
        match stmt {
            Statement::Expr(_) => {
                result = evaluator.eval(stmt.clone())?;
            }
            Statement::Return(_) => match evaluator.eval(stmt.clone())? {
                Some(result) => return Ok(result),
                None => return Err("Return statement without return value".to_string()),
            },
            _ => {
                evaluator.eval(stmt.clone())?;
            }
        }
    }
    result.ok_or_else(|| "Function return value is None".to_string())
}
