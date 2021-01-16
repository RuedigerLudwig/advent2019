use common::Direction;
use computer::{Code, ComputerError, ComputerInput, ListInput, Output, VirtualMachine};

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
    output: Output<ListInput>,
    input: ListInput,
}

impl ComputerInterface {
    pub fn new(code: &Code) -> ComputerInterface {
        let input = ListInput::new();
        let vm = VirtualMachine::with_input(code, input.clone());
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
    fn send_direction(&mut self, direction: Direction) -> Result<Report, ComputerError> {
        let dir_number = ComputerInterface::dir_number(direction);
        self.input.provide_input(dir_number);

        if let Some(report) = self.output.next() {
            match report? {
                0 => Ok(Report::Wall),
                1 => Ok(Report::Moved),
                2 => Ok(Report::Oxygen),
                code => Err(ComputerError::MessageError(format!(
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
