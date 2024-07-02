use super::prelude::*;

pub struct VM<'a> {
    stack: Vec<Value>,
    pc: Pointer,
    code: &'a Chunk,
}

impl VM<'_> {
    pub fn new(code: &Chunk) -> Self {
        VM {
            stack: Vec::new(),
            pc: 0,
            code,
        }
    }
    pub fn run(&mut self) {}

    pub fn print_stack(&self) {
        print!("          ");
        for value in &self.stack {
            print!("[{}]", value);
        }
        println!();
    }
}
