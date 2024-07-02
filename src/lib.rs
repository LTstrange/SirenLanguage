mod parser;
mod stack_vm;

use parser::*;
use stack_vm::*;

pub fn run_file(input: &str) -> Result<(), String> {
    let program = parser::parse_file(input)?;
    println!("{}", program);
    let code = compile(program).unwrap();
    VM::new(&code).run();
    Ok(())
}

pub fn run_line(input: &str) -> Result<(), String> {
    let item = parser::parse_line(input)?;
    if let Some(item) = item {
        println!("{}", item);
    }
    Ok(())
}
