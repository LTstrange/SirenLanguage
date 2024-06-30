// mod interpreter;
mod parser;

pub fn run_file(input: &str) -> Result<(), String> {
    let program = parser::parse_file(input)?;
    for item in program.0 {
        println!("{:?}", item);
    }
    Ok(())
}

pub fn run_line(input: &str) -> Result<(), String> {
    let item = parser::parse_line(input)?;
    println!("{:?}", item);
    Ok(())
}
