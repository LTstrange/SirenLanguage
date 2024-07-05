use super::prelude::*;

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

    pub fn run(&mut self) -> Result<Value, String> {
        while self.pc < self.code.len() {
            let op = &self.code[self.pc];
            match op {
                Inst::Const(ind) => {
                    self.stack.push(self.code.get_const(*ind as usize).clone());
                }
                Inst::Ret => {
                    let v = self.stack.pop().unwrap();
                    return Ok(v);
                }
                Inst::Neg => match self.stack.pop().unwrap() {
                    Value::Number(v) => self.stack.push(Value::Number(-v)),
                },
                Inst::Add | Inst::Sub | Inst::Div | Inst::Mul => binary_op(self, op),
            }
            self.pc += 1;
            self.print_stack();
        }
        Ok(self.stack.pop().unwrap())
    }

    pub fn print_stack(&self) {
        print!("          ");
        for value in &self.stack {
            print!("[{}]", value);
        }
        println!();
    }
}

fn binary_op(vm: &mut VM, op: &Inst) {
    let Value::Number(b) = vm.stack.pop().unwrap();
    let Value::Number(a) = vm.stack.pop().unwrap();
    let v = match op {
        Inst::Add => Value::Number(a + b),
        Inst::Sub => Value::Number(a - b),
        Inst::Mul => Value::Number(a * b),
        Inst::Div => Value::Number(a / b),
        _ => panic!("Invalid binary operation"),
    };
    vm.stack.push(v);
}
