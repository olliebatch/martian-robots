mod mission_instructions;
mod parser;
mod robots;

use crate::parser::parse_input_to_command;
use crate::robots::RobotPosition;
use std::collections::HashSet;
use std::error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    let command = parse_input_to_command(buffer.as_str())?;

    let mut scent_tracker: HashSet<RobotPosition> = HashSet::new();

    for robot in command.robots {
        let robot = robot.process_all_commands(&command.upper_right, &mut scent_tracker);
        robot.end_of_mission_report()
    }

    Ok(())
}
