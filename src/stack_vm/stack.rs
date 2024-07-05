use super::*;

pub struct VM<'a> {
    stack: Vec<Value>,
    pc: Pointer,
    code: &'a Chunk,
}

impl<'a> VM<'a> {
    pub fn new(code: &'a Chunk) -> Self {
        VM {
            stack: Vec::new(),
            pc: 0,
            code,
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
            }
            self.pc += 1;
            self.print_stack(op, self.code);
        }
        Ok(self.stack.clone())
    }

    pub fn print_stack(&self, op: &Inst, chunk: &Chunk) {
        print!("{:30} ", op.disassemble(chunk));
        for value in &self.stack {
            print!("[{}]", value);
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
