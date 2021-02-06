use std::{convert::TryFrom, fmt::Display, ops::Index};

use crate::error::ComputerError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddrMode {
    Absolut,
    Direct,
    Relative,
}

impl AddrMode {
    pub fn format<T: Display>(&self, value: T, width: usize) -> String {
        match *self {
            AddrMode::Absolut => format!(" {:>width$} ", value, width = width),
            AddrMode::Direct => format!("[{:>width$}]", value, width = width),
            AddrMode::Relative => format!("{{{:>width$}}}", value, width = width),
        }
    }
}

impl Default for AddrMode {
    fn default() -> Self {
        AddrMode::Absolut
    }
}

impl TryFrom<i64> for AddrMode {
    type Error = ComputerError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AddrMode::Absolut),
            1 => Ok(AddrMode::Direct),
            2 => Ok(AddrMode::Relative),
            _ => Err(ComputerError::UnknownMode(value)),
        }
    }
}

#[derive(Debug)]
pub struct AddrModes {
    _modes: Vec<AddrMode>,
}

impl AddrModes {
    pub fn analyze_instruction(instruction: i64) -> Result<(u8, AddrModes), ComputerError> {
        let op_code = (instruction % 100) as u8;
        Ok((op_code, AddrModes::new(instruction / 100)?))
    }

    fn new(instruction: i64) -> Result<AddrModes, ComputerError> {
        let mut _modes = Vec::new();
        let mut instruction = instruction;
        while instruction > 0 {
            _modes.push(AddrMode::try_from(instruction % 10)?);
            instruction /= 10;
        }
        Ok(AddrModes { _modes })
    }
}

impl Index<usize> for AddrModes {
    type Output = AddrMode;

    fn index(&self, index: usize) -> &Self::Output {
        self._modes.get(index).unwrap_or(&AddrMode::Absolut)
    }
}
