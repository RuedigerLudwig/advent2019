use crate::computer_error::ComputerError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddrMode {
    Absolut,
    Direct,
    Relative,
}

impl AddrMode {
    pub fn new(value: i64) -> Result<AddrMode, ComputerError> {
        match value {
            0 => Ok(AddrMode::Absolut),
            1 => Ok(AddrMode::Direct),
            2 => Ok(AddrMode::Relative),
            _ => Err(ComputerError::UnknownMode(value)),
        }
    }
}

impl Default for AddrMode {
    fn default() -> Self {
        AddrMode::Absolut
    }
}

pub struct AddrModes {
    _modes: Vec<AddrMode>,
}

impl AddrModes {
    pub fn new(instruction: i64) -> Result<AddrModes, ComputerError> {
        let mut _modes = Vec::new();
        let mut instruction = instruction;
        while instruction > 0 {
            _modes.push(AddrMode::new(instruction % 10)?);
            instruction /= 10;
        }
        Ok(AddrModes { _modes })
    }

    pub fn get(&self, pos: usize) -> AddrMode {
        self._modes.get(pos).copied().unwrap_or_default()
    }
}