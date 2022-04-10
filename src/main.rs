mod mission_instructions;
mod parser;
mod robots;

use crate::parser::parse_input_to_command;
use std::error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    let command = parse_input_to_command(buffer.as_str())?;
    println!("{:?}", command);

    let first_robot = command.robots[0].clone().process_all_commands();

    println!("{:?}", first_robot);
    Ok(())
}
