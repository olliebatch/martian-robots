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

    for robot in command.robots {
        let robot = robot.process_all_commands(&command.upper_right);
        println!("{}", robot.position)
    }

    Ok(())
}
