use common::Direction;
use computer::{computer_error::ComputerError, Computer};

#[derive(Debug)]
pub enum Report {
    Wall,
    Moved,
    Oxygen,
}

pub trait DroidComputerInterface {
    fn send_direction(&mut self, direction: Direction) -> Result<Report, ComputerError>;
}

pub struct ComputerInterface {
    computer: Computer,
}

impl ComputerInterface {
    pub fn new(computer: &Computer) -> ComputerInterface {
        ComputerInterface {
            computer: computer.clone(),
        }
    }

    fn dir_number(direction: Direction) -> i64 {
        match direction {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

impl DroidComputerInterface for ComputerInterface {
    fn send_direction(&mut self, direction: Direction) -> Result<Report, ComputerError> {
        let dir_number = ComputerInterface::dir_number(direction);
        self.computer.provide_input(dir_number);
        if let Some(report) = self.computer.next() {
            match report {
                Err(err) => Err(err),
                Ok(0) => Ok(Report::Wall),
                Ok(1) => Ok(Report::Moved),
                Ok(2) => Ok(Report::Oxygen),
                Ok(code) => Err(ComputerError::MessageError(format!(
                    "Unknown result code: {}",
                    code
                ))),
            }
        } else {
            Err(ComputerError::MessageError(String::from(
                "Unexpected end of transmission",
            )))
        }
    }
}
