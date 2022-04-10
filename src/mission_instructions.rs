use crate::robots::Robot;
use anyhow::anyhow;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Coordinates {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // remove any spaces if necessary
        let coords: String = s.chars().filter(|c| !c.is_whitespace()).collect();

        if coords.len() != 2 {
            //handle nicer error
            return Err(anyhow!("Too many characters for Coordinates"));
        }
        let mut chars = coords.chars();
        let x_from_char = chars.next().unwrap().to_string().parse::<i32>()?;
        let y_from_char = chars.next().unwrap().to_string().parse::<i32>()?;

        Ok(Coordinates {
            x: x_from_char,
            y: y_from_char,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub upper_right: Coordinates,
    pub robot_commands: Vec<Robot>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Orientation {
    North,
    South,
    West,
    East,
}

impl FromStr for Orientation {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "N" => Ok(Orientation::North),
            "S" => Ok(Orientation::South),
            "W" => Ok(Orientation::West),
            "E" => Ok(Orientation::East),
            _ => Err(anyhow!("Error matching orientation")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RobotCommands {
    Left,
    Right,
    Forward,
}

impl FromStr for RobotCommands {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "L" => Ok(RobotCommands::Left),
            "R" => Ok(RobotCommands::Right),
            "F" => Ok(RobotCommands::Forward),
            _ => Err(anyhow!("Error matching possible Robot Commands")),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mission_instructions::{Coordinates, Orientation, RobotCommands};
    use std::str::FromStr;

    use rstest::*;

    #[rstest]
    #[case("1 2", 1, 2)]
    #[case("12", 1, 2)]
    #[case("2 3", 2, 3)]
    #[case("23", 2, 3)]
    fn test_from_str_for_coords(
        #[case] input: &str,
        #[case] expected_x: i32,
        #[case] expected_y: i32,
    ) {
        let p = Coordinates::from_str(input);
        assert_eq!(
            p.unwrap(),
            Coordinates {
                x: expected_x,
                y: expected_y
            }
        )
    }

    #[rstest]
    #[case("N", Orientation::North)]
    #[case("W", Orientation::West)]
    #[case("E", Orientation::East)]
    #[case("S", Orientation::South)]
    fn test_orientation_from_str(#[case] input: &str, #[case] expected_orientation: Orientation) {
        let p = Orientation::from_str(input);
        assert_eq!(p.unwrap(), expected_orientation)
    }

    #[test]
    fn test_orientation_err_from_str() {
        let p = Orientation::from_str("Y");
        assert!(p.is_err())
    }

    #[rstest]
    #[case("F", RobotCommands::Forward)]
    #[case("L", RobotCommands::Left)]
    #[case("R", RobotCommands::Right)]
    fn test_robot_commands(#[case] input: &str, #[case] expected_command: RobotCommands) {
        let command = RobotCommands::from_str(input);

        assert_eq!(command.unwrap(), expected_command);
    }
}
