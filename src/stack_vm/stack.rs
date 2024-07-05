use std::collections::HashMap;

use super::*;

pub struct VM<'a> {
    stack: Vec<Value>,
    pc: Pointer,
    code: &'a Chunk,
    globals: HashMap<String, Value>,
}

impl<'a> VM<'a> {
    pub fn new(code: &'a Chunk) -> Self {
        VM {
            stack: Vec::new(),
            pc: 0,
            code,
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<Vec<Value>, RuntimeError> {
        while self.pc < self.code.len() {
            let op = &self.code[self.pc];
            match op {
                Inst::Const(ind) => {
                    self.stack.push(self.code.get_const(*ind as usize).clone());
                }
                Inst::Ret => {
                    return Ok(self.stack.clone());
                }
                Inst::Neg => match self.pop()? {
                    Value::Number(v) => self.stack.push(Value::Number(-v)),
                    _ => todo!("Invalid negation"),
                },
                Inst::Add | Inst::Sub | Inst::Div | Inst::Mul => binary_op(self, op)?,
                Inst::DefineGlobal(ind) => {
                    let Value::String(name) = self.code.get_const(*ind as usize) else {
                        return Err(RuntimeError::BadInstruction(
                            "Unwarp Ident, but not get string!!".to_string(),
                        ));
                    };
                    let value = self.pop()?;
                    self.globals.insert(name.clone(), value);
                }
                Inst::GetGlobal(ind) => {
                    let Value::String(name) = self.code.get_const(*ind as usize) else {
                        return Err(RuntimeError::BadInstruction(
                            "Unwrap Ident, but not get string!!".to_string(),
                        ));
                    };
                    if let Some(value) = self.globals.get(name) {
                        self.stack.push(value.clone());
                    } else {
                        return Err(RuntimeError::UndefinedVariable(name.to_string()));
                    }
                }
            }
            self.pc += 1;
            self.print_stack(op, self.code);
        }
        Ok(self.stack.clone())
    }

    pub fn print_stack(&self, op: &Inst, chunk: &Chunk) {
        print!("{:30} ", op.disassemble(chunk));
        for value in &self.stack {
            match value {
                Value::Number(n) => print!("[{}]", n),
                Value::String(s) => print!("[{:?}]", s),
                Value::Unit => print!("[()]"),
            }
        }
        println!();
    }

    fn pop(&mut self) -> Result<Value, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::StackUnderFlow)
    }
}

fn binary_op(vm: &mut VM, op: &Inst) -> Result<(), RuntimeError> {
    let Value::Number(b) = vm.pop()? else {
        return Err(RuntimeError::TypeMismatch("Expect Number".to_string()));
    };
    let Value::Number(a) = vm.pop()? else {
        return Err(RuntimeError::TypeMismatch("Expect Number".to_string()));
    };
    let v = match op {
        Inst::Add => Value::Number(a + b),
        Inst::Sub => Value::Number(a - b),
        Inst::Mul => Value::Number(a * b),
        Inst::Div => Value::Number(a / b),
        _ => panic!("Invalid binary operation"),
    };
    vm.stack.push(v);
    Ok(())
}
