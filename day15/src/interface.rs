use std::fmt::Display;

use common::Direction;
use computer::{Code, ComputerError, ListInput, Output, VirtualMachine};

use crate::error::DroidError;

#[derive(Debug)]
pub enum Report {
    Wall,
    Moved,
    Oxygen,
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Report::Wall => write!(f, "Wall"),
            Report::Moved => write!(f, "Moved"),
            Report::Oxygen => write!(f, "Oxygen"),
        }
    }
}

pub trait DroidComputerInterface {
    fn send_direction(&mut self, direction: Direction) -> Result<Report, DroidError>;
}

pub struct ComputerInterface {
    output: Output<ListInput>,
    input: ListInput,
}

impl ComputerInterface {
    pub fn new(code: &Code) -> ComputerInterface {
        let input = ListInput::new();
        let vm = VirtualMachine::new(code, &input);
        ComputerInterface {
            input,
            output: vm.get_output(),
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
    fn send_direction(&mut self, direction: Direction) -> Result<Report, DroidError> {
        let dir_number = ComputerInterface::dir_number(direction);
        self.input.provide_input(dir_number);

        if let Some(report) = self.output.next()? {
            match report {
                0 => Ok(Report::Wall),
                1 => Ok(Report::Moved),
                2 => Ok(Report::Oxygen),
                code => Err(DroidError::UnknownTile(code)),
            }
        } else {
            Err(ComputerError::OutputEmpty.into())
        }
    }
}
