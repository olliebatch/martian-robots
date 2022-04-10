use crate::mission_instructions::{Coordinates, Orientation, RobotCommands};

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

    pub fn move_forward(self) -> RobotPosition {
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
            orientation: self.orientation,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Robot {
    pub position: RobotPosition,
    pub robot_commands: Vec<RobotCommands>,
}

impl Robot {
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
}

#[cfg(test)]
mod test {

    use crate::mission_instructions::{Coordinates, Orientation};
    use crate::robots::RobotPosition;
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
}
