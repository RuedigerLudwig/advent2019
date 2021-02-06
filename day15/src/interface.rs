use std::fmt::Display;

use common::Direction;
use computer::{Code, ComputerError, ListInput, STVirtualMachine};

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

pub struct ComputerInterface<'a> {
    vm: STVirtualMachine<'a>,
    input: ListInput,
}

impl<'a> ComputerInterface<'a> {
    pub fn new(code: Code) -> ComputerInterface<'a> {
        let input = ListInput::new();
        let vm = STVirtualMachine::new_single(code, input.clone());
        ComputerInterface { vm, input }
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

impl DroidComputerInterface for ComputerInterface<'_> {
    fn send_direction(&mut self, direction: Direction) -> Result<Report, DroidError> {
        let dir_number = ComputerInterface::dir_number(direction);
        self.input.provide_input(dir_number);

        if let Some(report) = self.vm.next()? {
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
