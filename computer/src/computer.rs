use common::as_int;
use std::str::FromStr;

use crate::computer_error::ComputerError;

#[derive(Clone)]
pub struct Computer {
    _memory: Vec<i32>,
    _pointer: usize,
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
            })
        }
    }

    pub fn patch_memory(&mut self, addr: usize, value: i32) -> Result<(), ComputerError> {
        self.set_value(addr, value)
    }

    pub fn get_memory(&self) -> &Vec<i32> {
        &self._memory
    }

    pub fn run(&mut self) -> Result<(), ComputerError> {
        while let Some(opcode) = self._memory.get(self._pointer) {
            match opcode {
                1 => self.add(),
                2 => self.mul(),

                99 => break,

                _ => Err(ComputerError::UnknownOperation(*opcode)),
            }?;
        }

        Ok(())
    }

    fn get_addr(&self, addr: usize) -> Result<usize, ComputerError> {
        match self._memory.get(addr) {
            Some(addr) => Ok(*addr as usize),
            None => Err(ComputerError::IllegalAddress(
                addr,
                String::from("getting address"),
            )),
        }
    }

    fn get_value(&self, addr: usize) -> Result<i32, ComputerError> {
        let addr = self.get_addr(addr)?;
        match self._memory.get(addr) {
            Some(value) => Ok(*value),
            None => Err(ComputerError::IllegalAddress(
                addr,
                String::from("getting value"),
            )),
        }
    }

    fn set_value(&mut self, addr: usize, value: i32) -> Result<(), ComputerError> {
        match self._memory.get_mut(addr) {
            Some(cell) => {
                *cell = value;
                Ok(())
            }
            None => Err(ComputerError::IllegalAddress(
                addr,
                String::from("setting value"),
            )),
        }
    }

    fn add(&mut self) -> Result<(), ComputerError> {
        let op1 = self.get_value(self._pointer + 1)?;
        let op2 = self.get_value(self._pointer + 2)?;
        let result = self.get_addr(self._pointer + 3)?;

        self.set_value(result, op1 + op2)?;
        self._pointer += 4;

        Ok(())
    }

    fn mul(&mut self) -> Result<(), ComputerError> {
        let op1 = self.get_value(self._pointer + 1)?;
        let op2 = self.get_value(self._pointer + 2)?;
        let result = self.get_addr(self._pointer + 3)?;

        self.set_value(result, op1 * op2)?;
        self._pointer += 4;

        Ok(())
    }
}

impl FromStr for Computer {
    type Err = ComputerError;

    fn from_str(input: &str) -> Result<Computer, Self::Err> {
        let code = input.split(",").map(as_int).collect::<Result<_, _>>()?;
        Ok(Computer::new(code)?)
    }
}
