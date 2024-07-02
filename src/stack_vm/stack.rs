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
            match self.code[self.pc] {
                Inst::Const(ind) => {
                    self.stack.push(self.code.get_const(ind as usize).clone());
                }
                Inst::Ret => {
                    let v = self.stack.pop().unwrap();
                    return Ok(v);
                }
                Inst::Neg => match self.stack.pop().unwrap() {
                    Value::Number(v) => self.stack.push(Value::Number(-v)),
                },
            }
            self.pc += 1;
            self.print_stack();
        }
        Ok(Value::Number(42.))
    }

    pub fn print_stack(&self) {
        print!("          ");
        for value in &self.stack {
            print!("[{}]", value);
        }
        println!();
    }
}
