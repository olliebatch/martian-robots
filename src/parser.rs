use crate::mission_instructions::{
    Command, Coordinates, Orientation, RobotCommands, RobotInfo, RobotPosition,
};
use anyhow::anyhow;
use itertools::Itertools;
use std::error;
use std::str::{FromStr, Lines};

pub fn parse_input_to_command(commands: &str) -> Result<Command, Box<dyn error::Error>> {
    let mut lines_of_instruction = commands.lines();
    //todo add a check here to actually check the number of lines etc
    let coords = lines_of_instruction.next();
    // match until cover the lines so that can unwrap.
    //todo add check here to ensure max value is not over 50
    let coordinates = match coords {
        None => Err(anyhow!("No Upper right point provided.")),
        Some(values) => Ok(Coordinates::from_str(values)?),
    }?;

    let robots = parse_robot_commands(lines_of_instruction)?;

    let command = Command {
        upper_right: coordinates,
        robot_commands: robots,
    };

    Ok(command)
}

fn parse_robot_commands(lines: Lines) -> Result<Vec<RobotInfo>, anyhow::Error> {
    let trimmed_lines = remove_lines_and_whitespace(lines);

    // assume current structure will stay the same with 2 lines = one robot
    let robot_infos = generate_robots_from_strs(trimmed_lines)?;

    Ok(robot_infos)
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

fn generate_robots_from_strs(
    trimmed_strings: Vec<String>,
) -> Result<Vec<RobotInfo>, anyhow::Error> {
    let mut robots = vec![];
    for chunk in &trimmed_strings.into_iter().chunks(2) {
        let mut robot = RobotInfo::new();
        for (index, robot_info) in chunk.into_iter().enumerate() {
            if index == 0 {
                let coordinates = Coordinates::from_str(&robot_info[..2])?;
                let orientation = Orientation::from_str(&robot_info[2..3])?;
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
        let str = "5 3
        3 2 N
        FRRFLLFFRRFLL";

        let robot_commands = parse_input_to_command(str).unwrap();

        insta::assert_debug_snapshot!(robot_commands)
    }

    #[test]
    fn test_parse_multiple_commands() {
        let str = "5 3
        1 1 E
        RFRFRFRF

        3 2 N
        FRRFLLFFRRFLL
        
        0 3 W
        LLFFFLFLFL";

        let robot_commands = parse_input_to_command(str).unwrap();

        insta::assert_debug_snapshot!(robot_commands)
    }

    #[test]
    fn test_parsing_robot_commands() {
        let str = "
        3 2 N
        FRRFLLFFRRFLL";
        let lines = str.lines();

        let robot_commands = parse_robot_commands(lines).unwrap();

        insta::assert_debug_snapshot!(robot_commands)
    }

    #[test]
    fn test_parsing_multiple_robot_commands() {
        let str = "
        1 1 E
        RFRFRFRF
        3 2 N
        FRRFLLFFRRFLL";
        let lines = str.lines();

        let robot_commands = parse_robot_commands(lines).unwrap();

        insta::assert_debug_snapshot!(robot_commands)
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

    #[test]
    fn test_generate_robots_from_strs() {
        let robots =
            generate_robots_from_strs(vec!["32N".to_string(), "FRRFLLFFRRFLL".to_string()])
                .unwrap();

        insta::assert_debug_snapshot!(robots)
    }

    #[test]
    fn test_generate_robots_from_strs_two_robots() {
        let robots = generate_robots_from_strs(vec![
            "11E".to_string(),
            "RFRFRFRF".to_string(),
            "32N".to_string(),
            "FRRFLLFFRRFLL".to_string(),
        ])
        .unwrap();

        insta::assert_debug_snapshot!(robots)
    }
}
