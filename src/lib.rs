mod compiler;
mod parser;
mod stack_vm;

use compiler::*;
use parser::*;
use stack_vm::*;

pub enum SirenError {
    Parse(ParserError),
    Compile(String),
    Runtime(RuntimeError),
}

pub fn run_file(input: &str) -> Result<(), SirenError> {
    let program = parse_file(input).map_err(SirenError::Parse)?;
    println!("{}", program);
    let code = compile(program).map_err(SirenError::Compile)?;
    disassemble_chunk(&code, "Compiled Code");
    let result = VM::new(&code).run().map_err(SirenError::Runtime)?;
    println!("Output: {}", result);
    Ok(())
}

// pub fn run_line(input: &str) -> Result<(), String> {
//     let item = parse_line(input)?.unwrap();
//     let code = compile_item(item).unwrap();
//     let mut vm = VM::new(&code);
//     match vm.run() {
//         Ok(v) => {
//             println!("Output: {}", v);
//         }
//         Err(e) => {
//             return Err(format!("RuntimeError: {}", e));
//         }
//     }
//     Ok(())
// }
