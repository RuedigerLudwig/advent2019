use super::error::PaintError;
use crate::computer::{Code, Input, ListInput, VirtualMachine};
use crate::{common::turn::Turn, computer::ComputerError};
use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
};

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

impl TryFrom<i64> for Color {
    type Error = PaintError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Color::White),
            0 => Ok(Color::Black),
            _ => Err(PaintError::UnknownColor(value)),
        }
    }
}

impl Input for Color {
    fn get_data(&self) -> Result<Vec<i64>, ComputerError> {
        match self {
            Color::White => Ok(vec![1]),
            Color::Black => Ok(vec![0]),
        }
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
    fn accept_input(&mut self, color: Color) -> Result<Option<(Color, Turn)>, PaintError>;
}

pub struct ComputerInterface<'a> {
    vm: VirtualMachine<'a>,
    input: ListInput,
}

impl<'a> ComputerInterface<'a> {
    pub fn new(code: Code) -> ComputerInterface<'a> {
        let input = ListInput::new();
        let vm = VirtualMachine::new(code, input.clone());
        ComputerInterface { input, vm }
    }
}

impl BotComputerInterface for ComputerInterface<'_> {
    fn accept_input(&mut self, color: Color) -> Result<Option<(Color, Turn)>, PaintError> {
        self.input.provide(color)?;

        if let Some(output) = self.vm.take_exactly(2)? {
            let paint = output[0].try_into()?;

            let turn = match output[1] {
                0 => Turn::Left,
                1 => Turn::Right,
                turn => return Err(PaintError::UnknownTurn(turn)),
            };

            Ok(Some((paint, turn)))
        } else {
            Ok(None)
        }
    }
}
