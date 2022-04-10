use anyhow::anyhow;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    x: i32,
    y: i32,
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
    pub robot_commands: Vec<RobotInfo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RobotInfo {
    pub start_position: (Coordinates, Orientation),
    pub robot_commands: RobotCommands,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Orientation {
    North,
    South,
    West,
    East,
}
#[derive(Debug, Clone, PartialEq)]
pub enum RobotCommands {
    Left,
    Right,
    Forward,
}

#[cfg(test)]
mod test {
    use crate::mission_instructions::Coordinates;
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
}
