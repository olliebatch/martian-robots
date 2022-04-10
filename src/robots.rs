use crate::mission_instructions::{Coordinates, Orientation, RobotCommands};

#[derive(Debug, Clone, PartialEq)]
pub struct RobotPosition {
    pub coordinates: Coordinates,
    pub orientation: Orientation,
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
