use std::fmt::Display;

use common::Turn;
use computer::{Code, ComputerError, ComputerInput, ListInput, Output, VirtualMachine};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Color::Black => write!(f, " "),
            Color::White => write!(f, "#"),
        }
    }
}

pub trait BotComputerInterface {
    fn accept_input(&mut self, color: Color) -> Result<Option<(Color, Turn)>, ComputerError>;
}

pub struct ComputerInterface {
    output: Output<ListInput>,
    input: ListInput,
}

impl ComputerInterface {
    pub fn new(code: &Code) -> ComputerInterface {
        let input = ListInput::new();
        let vm = VirtualMachine::with_input(&code, input.clone());
        ComputerInterface {
            input,
            output: vm.get_output(),
        }
    }
}

impl BotComputerInterface for ComputerInterface {
    fn accept_input(&mut self, color: Color) -> Result<Option<(Color, Turn)>, ComputerError> {
        self.input
            .provide_input(if color == Color::White { 1 } else { 0 });

        if let Some(output) = self.output.take_exactly(2)? {
            let paint = match output[0] {
                0 => Color::Black,
                1 => Color::White,
                n => return Err(ComputerError::MessageError(format!("Unknown color: {}", n))),
            };

            let turn = match output[1] {
                0 => Turn::Left,
                1 => Turn::Right,
                n => return Err(ComputerError::MessageError(format!("Unknown turn: {}", n))),
            };

            Ok(Some((paint, turn)))
        } else {
            Ok(None)
        }
    }
}
