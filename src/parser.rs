use crate::mission_instructions::{Coordinates, Orientation, RobotPosition};
use anyhow::anyhow;
use itertools::Itertools;
use std::error;
use std::str::{FromStr, Lines};

fn parse_input_to_command(commands: &str) -> Result<(), Box<dyn error::Error>> {
    let mut lines_of_instruction = commands.lines();
    //todo add a check here to actually check the number of lines etc
    let coords = lines_of_instruction.next();
    // match until cover the lines so that can unwrap.
    //todo add check here to ensure max value is not over 50
    let coordinates = match coords {
        None => Err(anyhow!("No Upper right point provided.")),
        Some(values) => Ok(Coordinates::from_str(values)?),
    }?;

    Ok(())
}

fn parse_robot_commands(lines: Lines) -> Result<(), anyhow::Error> {
    let trimmed_lines = remove_lines_and_whitespace(lines);

    // assume current structure will stay the same with 2 lines = one robot
    for chunk in &trimmed_lines.into_iter().chunks(2) {
        for (index, robot_info) in chunk.into_iter().enumerate() {
            if index == 0 {
                let coordinates = Coordinates::from_str(&robot_info[..2])?;
                let orientation = Orientation::from_str(&robot_info[2..3])?;
                let robot_position = RobotPosition {
                    coordinates,
                    orientation,
                };
            } else {
                println!("index 1{:?}", robot_info);
            }
        }
    }

    Ok(())
}

fn remove_lines_and_whitespace(lines: Lines) -> Vec<String> {
    //todo if time how to handle a robot that's dropped with no movements?
    let mut removed_lines: Vec<String> = vec![];
    for line in lines {
        let removed_whitespace: String = line.chars().filter(|c| !c.is_whitespace()).collect();

        if removed_whitespace.is_empty() {
            // do nothing because it's an empty line.
        } else {
            removed_lines.push(removed_whitespace)
        }
    }
    removed_lines
}

#[cfg(test)]
mod test {
    use crate::parser::{parse_robot_commands, remove_lines_and_whitespace};

    #[test]
    fn test_parsing_robot_commands() {
        let str = "
        3 2 N
        FRRFLLFFRRFLL";
        let lines = str.lines();

        parse_robot_commands(lines).unwrap()
        //todo update when its returning
    }

    #[test]
    fn test_remove_lines_and_whitespace_space_and_lines() {
        let str = "
        3 2 N

        FRRFLLFFRRFLL";
        let lines = str.lines();

        let removed_lines = remove_lines_and_whitespace(lines);
        let expected = vec!["32N".to_string(), "FRRFLLFFRRFLL".to_string()];

        assert_eq!(removed_lines, expected)
    }

    #[test]
    fn test_remove_lines_and_whitespace_no_lines() {
        let str = "
        3 2 N
        FRRFLLFFRRFLL";
        let lines = str.lines();

        let removed_lines = remove_lines_and_whitespace(lines);
        let expected = vec!["32N".to_string(), "FRRFLLFFRRFLL".to_string()];

        assert_eq!(removed_lines, expected)
    }
}
