use crate::robots::{Robot, RobotPosition};
use anyhow::anyhow;
use std::fmt;
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
    pub robots: Vec<Robot>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Orientation {
    North,
    South,
    West,
    East,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            Orientation::North => "N",
            Orientation::South => "S",
            Orientation::West => "W",
            Orientation::East => "E",
        };
        write!(f, "{:}", val)
    }
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
impl Orientation {
    pub fn change_right(&self) -> Self {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }
    pub fn change_left(&self) -> Self {
        match self {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RobotCommands {
    Left,
    Right,
    Forward,
}

impl RobotCommands {
    pub fn process(&self, robot_position: RobotPosition) -> RobotPosition {
        match self {
            RobotCommands::Right => {
                let new_orientation = robot_position.orientation.change_right();
                robot_position.update_orientation(new_orientation)
            }
            RobotCommands::Left => {
                let new_orientation = robot_position.orientation.change_left();
                robot_position.update_orientation(new_orientation)
            }
            RobotCommands::Forward => robot_position.move_forward(),
        }
    }
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

    #[rstest]
    #[case(Orientation::North, Orientation::East)]
    #[case(Orientation::South, Orientation::West)]
    #[case(Orientation::East, Orientation::South)]
    #[case(Orientation::West, Orientation::North)]
    fn test_moving_right(#[case] input: Orientation, #[case] expected_orientation: Orientation) {
        let p = input.change_right();
        assert_eq!(p, expected_orientation)
    }
    #[rstest]
    #[case(Orientation::North, Orientation::West)]
    #[case(Orientation::South, Orientation::East)]
    #[case(Orientation::East, Orientation::North)]
    #[case(Orientation::West, Orientation::South)]
    fn test_moving_left(#[case] input: Orientation, #[case] expected_orientation: Orientation) {
        let p = input.change_left();
        assert_eq!(p, expected_orientation)
    }
}
