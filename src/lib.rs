mod parser;
mod stack_vm;

use parser::*;
use stack_vm::*;

pub fn run_file(input: &str) -> Result<(), String> {
    let program = parser::parse_file(input)?;
    println!("{}", program);
    let code = compile(program).unwrap();
    let result = VM::new(&code).run().unwrap();
    println!("Result: {}", result);
    Ok(())
}

pub fn run_line(input: &str) -> Result<(), String> {
    let item = parser::parse_line(input)?.unwrap();
    let code = compile_item(item).unwrap();
    let mut vm = VM::new(&code);
    vm.run();
    Ok(())
}
