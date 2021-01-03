use common::as_int;
use std::{collections::VecDeque, str::FromStr};

use crate::computer_error::ComputerError;

#[derive(Clone)]
pub struct Computer {
    _memory: Vec<i32>,
    _pointer: usize,
    _terminated: bool,
    _input: VecDeque<i32>,
}

impl Computer {
    pub fn new(code: Vec<i32>) -> Result<Computer, ComputerError> {
        if code.len() <= 0 {
            Err(ComputerError::MessageError(String::from(
                "No code was provided for this computer",
            )))
        } else {
            Ok(Computer {
                _memory: code.clone(),
                _pointer: 0,
                _terminated: false,
                _input: VecDeque::new(),
            })
        }
    }

    pub fn run(&mut self) -> Result<Vec<i32>, ComputerError> {
        self.collect()
    }

    pub fn patch_memory(&mut self, addr: usize, value: i32) -> Result<(), ComputerError> {
        self.set_value(addr, value)
    }

    pub fn get_memory(&self) -> &Vec<i32> {
        &self._memory
    }

    pub fn provide_input(&mut self, value: i32) {
        self._input.push_back(value)
    }

    fn get_addr(&self, addr: usize, mode: u8) -> Result<usize, ComputerError> {
        match mode {
            0 => {
                if let Some(addr) = self._memory.get(addr) {
                    if *addr < 0 {
                        Err(ComputerError::IllegalAddress(format!(
                            "{} for getting address",
                            addr
                        )))
                    } else {
                        Ok(*addr as usize)
                    }
                } else {
                    Err(ComputerError::IllegalAddress(format!(
                        "{} for getting address",
                        addr
                    )))
                }
            }

            1 => Ok(addr),
            _ => Err(ComputerError::IllegalMode(mode)),
        }
    }

    fn get_value(&self, addr: usize, mode: u8) -> Result<i32, ComputerError> {
        let addr = match mode {
            0 => self.get_addr(addr, 0)?,
            1 => addr,

            _ => Err(ComputerError::IllegalMode(mode))?,
        };

        if let Some(value) = self._memory.get(addr) {
            Ok(*value)
        } else {
            Err(ComputerError::IllegalAddress(format!(
                "{} for getting value",
                addr
            )))
        }
    }

    fn get_value_as_address(&self, addr: usize, mode: u8) -> Result<usize, ComputerError> {
        let addr = self.get_value(addr, mode)?;
        if addr < 0 {
            Err(ComputerError::IllegalAddress(format!(
                "{} for value address",
                addr
            )))
        } else {
            Ok(addr as usize)
        }
    }

    fn set_value(&mut self, addr: usize, value: i32) -> Result<(), ComputerError> {
        match self._memory.get_mut(addr) {
            Some(cell) => {
                *cell = value;
                Ok(())
            }
            None => Err(ComputerError::IllegalAddress(format!(
                "{} for setting value",
                addr
            ))),
        }
    }

    fn analyze_instruction(&self, instruction: i32) -> (i32, Modes) {
        let op_code = instruction % 100;
        let mut modes = Vec::new();
        let mut instruction = instruction / 100;
        while instruction > 0 {
            modes.push((instruction % 10) as u8);
            instruction /= 10;
        }
        (op_code, Modes { modes })
    }

    fn add(&mut self, modes: &Modes) -> Result<Option<i32>, ComputerError> {
        let op1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let op2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;

        self.set_value(addr, op1 + op2)?;
        self._pointer += 4;

        Ok(None)
    }

    fn mul(&mut self, modes: &Modes) -> Result<Option<i32>, ComputerError> {
        let op1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let op2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;

        self.set_value(addr, op1 * op2)?;
        self._pointer += 4;

        Ok(None)
    }

    fn input(&mut self, modes: &Modes) -> Result<Option<i32>, ComputerError> {
        if let Some(input) = self._input.pop_front() {
            let addr = self.get_addr(self._pointer + 1, modes.get(0))?;
            self.set_value(addr, input)?;
            self._pointer += 2;

            Ok(None)
        } else {
            Err(ComputerError::InputEmpty)
        }
    }

    fn output(&mut self, modes: &Modes) -> Result<Option<i32>, ComputerError> {
        let addr = self.get_addr(self._pointer + 1, modes.get(0))?;
        let value = self.get_value(addr, 1)?;
        self._pointer += 2;
        Ok(Some(value))
    }

    fn jump_non_zero(&mut self, modes: &Modes) -> Result<Option<i32>, ComputerError> {
        let cmp = self.get_value(self._pointer + 1, modes.get(0))?;
        self._pointer = if cmp != 0 {
            self.get_value_as_address(self._pointer + 2, modes.get(1))?
        } else {
            self._pointer + 3
        };

        Ok(None)
    }

    fn jump_zero(&mut self, modes: &Modes) -> Result<Option<i32>, ComputerError> {
        let cmp = self.get_value(self._pointer + 1, modes.get(0))?;
        self._pointer = if cmp == 0 {
            self.get_value_as_address(self._pointer + 2, modes.get(1))?
        } else {
            self._pointer + 3
        };

        Ok(None)
    }

    fn less_than(&mut self, modes: &Modes) -> Result<Option<i32>, ComputerError> {
        let cmp1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let cmp2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;
        self.set_value(addr, if cmp1 < cmp2 { 1 } else { 0 })?;
        self._pointer += 4;
        Ok(None)
    }

    fn equals(&mut self, modes: &Modes) -> Result<Option<i32>, ComputerError> {
        let cmp1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let cmp2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;
        self.set_value(addr, if cmp1 == cmp2 { 1 } else { 0 })?;
        self._pointer += 4;
        Ok(None)
    }
}

struct Modes {
    modes: Vec<u8>,
}

impl Modes {
    pub fn get(&self, pos: usize) -> u8 {
        *self.modes.get(pos).unwrap_or(&0)
    }
}

impl Iterator for Computer {
    type Item = Result<i32, ComputerError>;

    fn next(&mut self) -> Option<Result<i32, ComputerError>> {
        if self._terminated {
            return None;
        }

        while let Some(instruction) = self._memory.get(self._pointer) {
            let (opcode, modes) = self.analyze_instruction(*instruction);

            let result = match opcode {
                1 => self.add(&modes),
                2 => self.mul(&modes),
                3 => self.input(&modes),
                4 => self.output(&modes),
                5 => self.jump_non_zero(&modes),
                6 => self.jump_zero(&modes),
                7 => self.less_than(&modes),
                8 => self.equals(&modes),
                99 => {
                    self._terminated = true;
                    return None;
                }

                _ => Err(ComputerError::UnknownOperation(opcode)),
            };

            match result {
                Err(error) => {
                    self._terminated = true;
                    return Some(Err(error));
                }
                Ok(Some(value)) => return Some(Ok(value)),
                Ok(None) => (),
            }
        }

        self._terminated = true;
        Some(Err(ComputerError::IllegalAddress(format!(
            "{} for next op code",
            self._pointer
        ))))
    }
}

impl FromStr for Computer {
    type Err = ComputerError;

    fn from_str(input: &str) -> Result<Computer, Self::Err> {
        let code = input.split(",").map(as_int).collect::<Result<_, _>>()?;
        Ok(Computer::new(code)?)
    }
}
