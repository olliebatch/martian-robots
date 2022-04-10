use crate::mission_instructions::{Command, Coordinates, Orientation, RobotCommands};
use crate::robots::{Robot, RobotPosition};
use anyhow::anyhow;
use itertools::Itertools;
use std::error;
use std::str::{FromStr, Lines};

pub fn parse_input_to_command(commands: &str) -> Result<Command, Box<dyn error::Error>> {
    println!("{:?}", commands);
    let mut lines_of_instruction = commands.lines();
    //todo add a check here to actually check the number of lines etc
    let coords = lines_of_instruction.next();
    // match until cover the lines so that can unwrap.
    let coordinates = match coords {
        None => Err(anyhow!("No Upper right point provided.")),
        Some(values) => Ok(Coordinates::from_str(values)?),
    }?;

    coordinates.check_max_value()?;

    let robots = parse_robot_commands(lines_of_instruction)?;

    let command = Command {
        upper_right: coordinates,
        robots,
    };

    Ok(command)
}

fn parse_robot_commands(lines: Lines) -> Result<Vec<Robot>, anyhow::Error> {
    let trimmed_lines = remove_lines_and_whitespace(lines);

    // assume current structure will stay the same with 2 lines = one robot
    let robot_infos = generate_robots_from_strs(trimmed_lines)?;

    Ok(robot_infos)
}

fn remove_lines_and_whitespace(lines: Lines) -> Vec<String> {
    //todo if time how to handle a robot that's dropped with no movements?
    let mut removed_lines: Vec<String> = vec![];
    for line in lines {
        if line.is_empty() {
            // do nothing because it's an empty line.
        } else {
            removed_lines.push(line.to_string())
        }
    }
    removed_lines
}

fn generate_robots_from_strs(trimmed_strings: Vec<String>) -> Result<Vec<Robot>, anyhow::Error> {
    let mut robots = vec![];
    for chunk in &trimmed_strings.into_iter().chunks(2) {
        let mut robot = Robot::new();
        for (index, robot_info) in chunk.into_iter().enumerate() {
            if index == 0 {
                let info = robot_info.rsplit_once(' ').unwrap();

                println!("{:?}", info);
                let coordinates = Coordinates::from_str(info.0)?;
                let orientation = Orientation::from_str(info.1)?;
                let robot_position = RobotPosition {
                    coordinates,
                    orientation,
                };
                robot = robot.set_start_position(robot_position);
            } else {
                let robot_commands: Result<Vec<RobotCommands>, anyhow::Error> = robot_info
                    .chars()
                    .map(|test| RobotCommands::from_str(test.to_string().as_str()))
                    .collect();
                robot = robot.update_commands(robot_commands?);
            }
        }
        robots.push(robot)
    }

    Ok(robots)
}

#[cfg(test)]
mod test {
    use crate::parser::{
        generate_robots_from_strs, parse_input_to_command, parse_robot_commands,
        remove_lines_and_whitespace,
    };

    #[test]
    fn test_parse_input_commands() {
        let str = "5 3\n3 2 N\nFRRFLLFFRRFLL";

        let robot_commands = parse_input_to_command(str).unwrap();

        insta::assert_debug_snapshot!(robot_commands)
    }

    #[test]
    fn test_error_over_50() {
        let str = "51 51\n3 2 N\nFRRFLLFFRRFLL";

        let robot_commands = parse_input_to_command(str);

        insta::assert_debug_snapshot!(robot_commands)
    }

    #[test]
    fn test_parse_multiple_commands() {
        let str = "5 3\n1 1 E\nRFRFRFRF\n\n3 2 N\nFRRFLLFFRRFLL\n\n0 3 W\nLLFFFLFLFL";

        let robot_commands = parse_input_to_command(str).unwrap();

        insta::assert_debug_snapshot!(robot_commands)
    }

    #[test]
    fn test_parsing_robot_commands() {
        let str = "3 2 N\nFRRFLLFFRRFLL";
        let lines = str.lines();

        let robot_commands = parse_robot_commands(lines).unwrap();

        insta::assert_debug_snapshot!(robot_commands)
    }

    #[test]
    fn test_parsing_multiple_robot_commands() {
        let str = "1 1 E\nRFRFRFRF\n\n3 2 N\nFRRFLLFFRRFLL";
        let lines = str.lines();

        let robot_commands = parse_robot_commands(lines).unwrap();

        insta::assert_debug_snapshot!(robot_commands)
    }

    #[test]
    fn test_remove_lines_and_whitespace_space_and_lines() {
        let str = "3 2 N\nFRRFLLFFRRFLL";
        let lines = str.lines();

        let removed_lines = remove_lines_and_whitespace(lines);
        let expected = vec!["3 2 N".to_string(), "FRRFLLFFRRFLL".to_string()];

        assert_eq!(removed_lines, expected)
    }

    #[test]
    fn test_remove_lines_and_whitespace_no_lines() {
        let str = "3 2 N\nFRRFLLFFRRFLL";
        let lines = str.lines();

        let removed_lines = remove_lines_and_whitespace(lines);
        let expected = vec!["3 2 N".to_string(), "FRRFLLFFRRFLL".to_string()];

        assert_eq!(removed_lines, expected)
    }

    #[test]
    fn test_generate_robots_from_strs() {
        let robots =
            generate_robots_from_strs(vec!["3 2 N".to_string(), "FRRFLLFFRRFLL".to_string()])
                .unwrap();

        insta::assert_debug_snapshot!(robots)
    }

    #[test]
    fn test_generate_robots_from_strs_two_robots() {
        let robots = generate_robots_from_strs(vec![
            "1 1 E".to_string(),
            "RFRFRFRF".to_string(),
            "3 2 N".to_string(),
            "FRRFLLFFRRFLL".to_string(),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(robots)
    }
}
