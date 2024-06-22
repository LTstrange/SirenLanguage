mod value;

use std::{collections::HashMap, fmt::Display, iter::zip};

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

pub struct Environment {
    env_stack: Vec<HashMap<String, Value>>,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, stack) in self.env_stack.iter().enumerate() {
            writeln!(f, "=========================")?;
            writeln!(f, "Stack {}:", i)?;
            for (key, value) in stack {
                writeln!(f, "{} = {}", key, value)?;
            }
        }
        writeln!(f, "=========================")?;

        Ok(())
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            env_stack: vec![HashMap::new()],
        }
    }
}

impl Environment {
    // push a new environment for block_stmt evaluation.
    fn push(&mut self) {
        self.env_stack.push(HashMap::new());
    }

    // pop a environment when exit block_stmt evaluation.
    fn pop(&mut self) {
        self.env_stack.pop();
    }

    // bind variable to the variable table
    fn bind(&mut self, name: &str, value: Value) -> Result<(), String> {
        // binding only affects the top/current environment
        let last = self.env_stack.last_mut().unwrap();
        // check if the variable is already bound
        if last.contains_key(name) {
            return Err("Variable already exists".to_string());
        }
        last.insert(name.to_string(), value);
        Ok(())
    }

    // set(update) variable in the variable table
    fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
        // loop through all environments to find the variable. (from top to bottom)
        for env in self.env_stack.iter_mut().rev() {
            if env.contains_key(name) {
                env.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(format!("Variable {} not found", name))
    }

    // get variable from the variable table
    // todo : the return type should be a reference, but I'll do it later
    fn get(&self, name: &str) -> Result<Value, String> {
        // loop through all environments to find the variable. (from top to bottom)
        for env in self.env_stack.iter().rev() {
            if env.contains_key(name) {
                return Ok(env[name].clone());
            }
        }
        Err(format!("Variable {} not found", name))
    }
}

pub struct Evaluator {
    // todo : solve "return in block" problem, use a cleaner way to return value
    pub env: Environment,
    ret_value: Option<Value>,
}

impl Evaluator {
    pub fn new(self_func: Option<(&str, Vec<String>, Vec<Statement>)>) -> Self {
        let mut env = Environment::default();
        if let Some((self_func, params, body)) = self_func {
            env.bind(self_func, Value::Fn { params, body }).unwrap();
        }
        Self {
            env,
            ret_value: None,
        }
    }

    fn update_with_args(&mut self, params: Vec<&str>, args: Vec<Value>) {
        for (param, value) in zip(params, args) {
            self.env.bind(param, value).unwrap();
        }
    }

    // evaluate oneline code
    pub fn eval(&mut self, ast: &Statement) -> Result<Value, String> {
        match ast {
            Statement::Let { name, value } => {
                let value = self.eval_expr(value)?;
                self.env.bind(name, value)?;
                Ok(Value::Unit)
            }
            Statement::Expr(expr) => self.eval_expr(expr),
            Statement::Set { name, value } => {
                let value = self.eval_expr(value)?;
                self.env.set(name, value)?;
                Ok(Value::Unit)
            }
            Statement::Return(ret) => {
                self.ret_value = Some(self.eval_expr(ret)?);
                Ok(Value::Unit)
            }
        }
    }

    // evaluate expression
    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Ident(ident) => self.env.get(ident),
            Expr::Literal(literal) => match literal {
                Literal::Int(n) => Ok(Value::Int(*n)),
                Literal::Bool(b) => Ok(Value::Bool(*b)),
            },
            Expr::Function { body, typed_params } => Ok(Value::Fn {
                params: typed_params.iter().map(|p| p.1.to_string()).collect(),
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
                    // call on func identity
                    Expr::Ident(func_name) => {
                        let func = self.env.get(func_name)?;
                        let (params, body) = match func {
                            Value::Fn { params, body } => Ok((params, body)),
                            _ => Err("this is not a function".to_string()),
                        }?;

                        eval_func(&params, args, &body, Some(func_name))
                    }
                    // instant call on anonymous function
                    Expr::Function { typed_params, body } => eval_func(
                        &typed_params
                            .iter()
                            .map(|p| p.1.to_owned())
                            .collect::<Vec<String>>(),
                        args,
                        body,
                        None,
                    ),
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
        self.env.push();
        let mut result = Value::Unit;
        for stmt in block {
            result = self.eval(stmt)?;
            if let Some(ret) = self.ret_value.clone() {
                return Ok(ret);
            }
        }
        self.env.pop();
        Ok(result)
    }
}

fn eval_func(
    params: &Vec<String>,
    args: Vec<Value>,
    body: &Vec<Statement>,
    self_func: Option<&str>,
) -> Result<Value, String> {
    // check whether the args_len matches the params_len
    if args.len() != params.len() {
        return Err(format!(
            "expect {} parameters, found {} args.",
            params.len(),
            args.len()
        ));
    }

    let mut evaluator = Evaluator::new(None);
    // bind self recursion
    if let Some(name) = self_func {
        evaluator = Evaluator::new(Some((name, params.to_owned(), body.to_owned())));
    }

    // bind parameters and arguments to the environment
    evaluator.update_with_args(
        params.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
        args,
    );

    let mut result = Value::Unit;
    for stmt in body {
        result = evaluator.eval(stmt)?;
        if let Some(ret) = evaluator.ret_value {
            return Ok(ret);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! build_siren_function {
        ($program: literal) => {{
            let stmt: Statement = SirenParser::parse_line($program).unwrap();
            match stmt {
                Statement::Let {
                    name,
                    value: Expr::Function { typed_params, body },
                } => (name, typed_params, body),
                _ => unreachable!(),
            }
        }};
    }

    macro_rules! test_siren_function_input_output {
        ($program: literal, $(   $input:expr    => $output: expr   ),+ $(,)?) => {
            let (name, params, body) = build_siren_function!($program);
            $(
                let result = eval_func(&params.iter()
                .map(|p| p.1.to_owned())
                .collect::<Vec<String>>(), $input, &body, Some(&name));
                match result {
                    Ok(Value::Int(n)) => assert_eq!(n, $output),
                    _ => unreachable!(),
                }
            )+
        };
    }

    #[test]
    fn test_abs() {
        test_siren_function_input_output!(
            "let abs = fn (x) {
                if x > 0 {x} else { -x}
            }",
            vec![Value::Int(0)] => 0,
            vec![Value::Int(-5)] => 5,
            vec![Value::Int(15)] => 15,

        );
    }

    #[test]
    fn test_fib() {
        // 1 1 2 3 5 8 13 21 34 55 89
        test_siren_function_input_output!(
            "let fib = fn (n) {
                let ans = 0;
                if n <= 1 {
                    return 1;
                } else {
                    ans = fib(n - 1) + fib(n - 2);
                };
                ans
            }",
            vec![Value::Int(0)] => 1,
            vec![Value::Int(1)] => 1,
            vec![Value::Int(2)] => 2,
            vec![Value::Int(3)] => 3,
            vec![Value::Int(4)] => 5,
            vec![Value::Int(5)] => 8,
            vec![Value::Int(6)] => 13,
            vec![Value::Int(7)] => 21,
            vec![Value::Int(8)] => 34,
            vec![Value::Int(9)] => 55,
        );
    }

    #[test]
    fn test_branch() {
        test_siren_function_input_output!(
            "let max = fn (a, b) {
                if a > b {a} else {b}
            }",
            vec![Value::Int(0), Value::Int(1)] => 1,
            vec![Value::Int(76), Value::Int(3)] => 76,
            vec![Value::Int(-2), Value::Int(-5)] => -2,
        );
    }
}
