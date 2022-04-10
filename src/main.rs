mod mission_instructions;
mod parser;
mod robots;

use crate::parser::parse_input_to_command;
use crate::robots::RobotPosition;
use anyhow::anyhow;
use std::collections::HashSet;
use std::error;
use std::io::{self, Read};
use std::str;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buffer = vec![];
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_end(&mut buffer)?;
    let string = match str::from_utf8(&buffer) {
        Ok(v) => Ok(v),
        Err(e) => Err(anyhow!("Invalid UTF-8 sequence: {}", e)),
    }?;

    let command = parse_input_to_command(string)?;

    let mut scent_tracker: HashSet<RobotPosition> = HashSet::new();

    for robot in command.robots {
        let robot = robot.process_all_commands(&command.upper_right, &mut scent_tracker);
        robot.end_of_mission_report()
    }

    Ok(())
}
