use crate::mission_instructions::{Coordinates, Orientation, RobotCommands};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RobotPosition {
    pub coordinates: Coordinates,
    pub orientation: Orientation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RobotStatus {
    Alive,
    Lost,
}

impl fmt::Display for RobotStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RobotStatus::Alive => write!(f, "{:}", "ALIVE"),
            RobotStatus::Lost => write!(f, "{:}", "LOST"),
        }
    }
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

    pub fn check_scent(&self, scent_tracker: &mut HashSet<RobotPosition>) -> bool {
        scent_tracker.contains(&self)
    }

    pub fn add_scent(&self, scent_tracker: &mut HashSet<RobotPosition>) {
        scent_tracker.insert(self.to_owned());
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
    pub robot_status: RobotStatus,
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
            robot_status: RobotStatus::Alive,
        }
    }

    pub fn new() -> Self {
        Robot {
            position: RobotPosition {
                coordinates: Coordinates { x: 0, y: 0 },
                orientation: Orientation::North,
            },
            robot_commands: vec![],
            robot_status: RobotStatus::Alive,
        }
    }

    pub fn set_start_position(self, robot_position: RobotPosition) -> Self {
        Robot {
            position: robot_position,
            robot_commands: self.robot_commands,
            robot_status: self.robot_status,
        }
    }

    pub fn update_commands(self, robot_commands: Vec<RobotCommands>) -> Self {
        Robot {
            position: self.position,
            robot_commands,
            robot_status: self.robot_status,
        }
    }

    pub fn process_robot_command(
        mut self,
        coordinate_limit: &Coordinates,
        scent_tracker: &mut HashSet<RobotPosition>,
    ) -> Self {
        let command = self.robot_commands.first().unwrap();
        let (new_position, robot_status) =
            command.process(self.position, coordinate_limit, scent_tracker);
        self.robot_commands.remove(0);

        Robot {
            position: new_position,
            robot_commands: self.robot_commands,
            robot_status,
        }
    }

    pub fn process_all_commands(
        mut self,
        coordinate_limit: &Coordinates,
        scent_tracker: &mut HashSet<RobotPosition>,
    ) -> Self {
        while !self.robot_commands.is_empty() && self.robot_status == RobotStatus::Alive {
            self = self.process_robot_command(coordinate_limit, scent_tracker);
        }
        self
    }

    pub fn end_of_mission_report(&self) {
        if self.robot_status == RobotStatus::Lost {
            println!("{} {}", self.position, self.robot_status)
        } else {
            println!("{}", self.position)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mission_instructions::{Coordinates, Orientation, RobotCommands};
    use crate::robots::{Robot, RobotPosition};
    use rstest::*;
    use std::collections::HashSet;

    #[rstest]
    #[case(Orientation::North, "north")]
    #[case(Orientation::South, "south")]
    #[case(Orientation::West, "west")]
    #[case(Orientation::East, "east")]
    fn test_from_str_for_coords(#[case] orientation: Orientation, #[case] snapshot_suffix: &str) {
        let mut settings = insta::Settings::new();
        settings.set_snapshot_suffix(snapshot_suffix);
        let robot_position = RobotPosition {
            coordinates: Coordinates { x: 2, y: 2 },
            orientation,
        };

        let moved = robot_position.move_forward();
        settings.bind(|| {
            // runs the assertion with the changed settings enabled
            insta::assert_debug_snapshot!(moved)
        });
    }

    #[rstest]
    #[case(RobotCommands::Forward, "test_single_rob_command_forward")]
    #[case(RobotCommands::Left, "test_single_rob_command_left")]
    #[case(RobotCommands::Right, "test_single_rob_command_right")]
    fn test_single_rob_command(#[case] command: RobotCommands, #[case] snapshot_suffix: &str) {
        let mut settings = insta::Settings::new();
        settings.set_snapshot_suffix(snapshot_suffix);

        let mut scent_tracker: HashSet<RobotPosition> = HashSet::new();
        let robot = Robot::new();
        let coordinates = Coordinates { x: 5, y: 3 };
        let robot_with_commands = robot.update_commands(vec![command]);
        let processed_robot =
            robot_with_commands.process_robot_command(&coordinates, &mut scent_tracker);
        settings.bind(|| {
            // runs the assertion with the changed settings enabled
            insta::assert_debug_snapshot!(processed_robot)
        });
    }

    #[test]
    fn test_process_all_commands() {
        let mut scent_tracker: HashSet<RobotPosition> = HashSet::new();
        let coordinates = Coordinates { x: 5, y: 3 };
        let new_basic_robot = Robot::new_basic_robot();
        let processed_robot =
            new_basic_robot.process_all_commands(&coordinates, &mut scent_tracker);
        insta::assert_debug_snapshot!(processed_robot)
    }
}
