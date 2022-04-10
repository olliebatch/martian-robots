use crate::mission_instructions::{Coordinates, Orientation, RobotCommands};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct RobotPosition {
    pub coordinates: Coordinates,
    pub orientation: Orientation,
}

impl RobotPosition {
    pub fn update_orientation(self, orientation: Orientation) -> RobotPosition {
        RobotPosition {
            coordinates: self.coordinates,
            orientation,
        }
    }

    pub fn move_forward(&self) -> Self {
        let new_coordinates = match self.orientation {
            Orientation::North => Coordinates {
                x: self.coordinates.x,
                y: self.coordinates.y + 1,
            },
            Orientation::South => Coordinates {
                x: self.coordinates.x,
                y: self.coordinates.y - 1,
            },
            Orientation::East => Coordinates {
                x: self.coordinates.x + 1,
                y: self.coordinates.y,
            },
            Orientation::West => Coordinates {
                x: self.coordinates.x - 1,
                y: self.coordinates.y,
            },
        };
        RobotPosition {
            coordinates: new_coordinates,
            orientation: self.orientation.to_owned(),
        }
    }
}

impl fmt::Display for RobotPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:} {:} {:}",
            self.coordinates.x, self.coordinates.y, self.orientation
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Robot {
    pub position: RobotPosition,
    pub robot_commands: Vec<RobotCommands>,
}

impl Robot {
    #[cfg(test)]
    pub fn new_basic_robot() -> Self {
        let robot_position = RobotPosition {
            coordinates: Coordinates { x: 1, y: 1 },
            orientation: Orientation::East,
        };
        let robot_commands = vec![
            RobotCommands::Right,
            RobotCommands::Forward,
            RobotCommands::Right,
            RobotCommands::Forward,
            RobotCommands::Right,
            RobotCommands::Forward,
            RobotCommands::Right,
            RobotCommands::Forward,
        ];
        Robot {
            robot_commands,
            position: robot_position,
        }
    }

    pub fn new() -> Self {
        Robot {
            position: RobotPosition {
                coordinates: Coordinates { x: 0, y: 0 },
                orientation: Orientation::North,
            },
            robot_commands: vec![],
        }
    }

    pub fn set_start_position(self, robot_position: RobotPosition) -> Self {
        Robot {
            position: robot_position,
            robot_commands: self.robot_commands,
        }
    }

    pub fn update_commands(self, robot_commands: Vec<RobotCommands>) -> Self {
        Robot {
            position: self.position,
            robot_commands,
        }
    }

    pub fn process_robot_command(mut self, coordinate_limit: &Coordinates) -> Self {
        let command = self.robot_commands.first().unwrap();
        let new_position = command.process(self.position, coordinate_limit);
        self.robot_commands.remove(0);
        Robot {
            position: new_position,
            robot_commands: self.robot_commands,
        }
    }

    pub fn process_all_commands(mut self, coordinate_limit: &Coordinates) -> Self {
        while !self.robot_commands.is_empty() {
            self = self.process_robot_command(coordinate_limit)
        }
        self
    }
}

#[cfg(test)]
mod test {

    use crate::mission_instructions::{Coordinates, Orientation, RobotCommands};
    use crate::robots::{Robot, RobotPosition};
    use rstest::*;

    #[rstest]
    #[case(Orientation::North)]
    #[case(Orientation::South)]
    #[case(Orientation::West)]
    #[case(Orientation::East)]
    fn test_from_str_for_coords(#[case] orientation: Orientation) {
        let robot_position = RobotPosition {
            coordinates: Coordinates { x: 2, y: 2 },
            orientation,
        };

        let moved = robot_position.move_forward();
        insta::assert_debug_snapshot!(moved)
    }

    #[rstest]
    #[case(RobotCommands::Forward)]
    #[case(RobotCommands::Left)]
    #[case(RobotCommands::Right)]

    fn test_single_rob_command(#[case] command: RobotCommands) {
        let robot = Robot::new();
        let robot_with_commands = robot.update_commands(vec![command]);
        let processed_robot = robot_with_commands.process_robot_command();
        insta::assert_debug_snapshot!(processed_robot)
    }

    #[test]
    fn test_process_all_commands() {
        let new_basic_robot = Robot::new_basic_robot();
        let processed_robot = new_basic_robot.process_all_commands();
        insta::assert_debug_snapshot!(processed_robot)
    }
}
