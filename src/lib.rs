mod compiler;
mod parser;
mod stack_vm;

use compiler::*;
use parser::*;
use stack_vm::*;

pub fn run_file(input: &str) -> Result<(), String> {
    let program = parse_file(input)?;
    println!("{}", program);
    let code = compile(program).unwrap();
    let result = VM::new(&code).run().unwrap();
    println!("Output: {}", result);
    Ok(())
}

pub fn run_line(input: &str) -> Result<(), String> {
    let item = parse_line(input)?.unwrap();
    let code = compile_item(item).unwrap();
    let mut vm = VM::new(&code);
    match vm.run() {
        Ok(v) => {
            println!("Output: {}", v);
        }
        Err(e) => {
            return Err(format!("RuntimeError: {}", e));
        }
    }
    Ok(())
}
