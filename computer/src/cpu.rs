use std::collections::HashMap;

use crate::{
    computer_error::ComputerError,
    input::ComputerInput,
    modes::{AddrMode, AddrModes},
};

#[derive(Debug)]
pub struct Cpu<I> {
    _memory: HashMap<usize, i64>,
    _relative_base: i64,
    _pointer: usize,
    _crashed: bool,
    _input: I,
}

enum OperationResult {
    Proceed,
    Exit,
    Output(i64),
}
use OperationResult::*;

impl<I> Cpu<I>
where
    I: ComputerInput,
{
    pub fn create(code: &[i64], input: I) -> Cpu<I> {
        let mut _memory = HashMap::new();
        for (addr, inst) in code.iter().enumerate() {
            _memory.insert(addr, *inst);
        }

        Cpu {
            _memory,
            _relative_base: 0,
            _pointer: 0,
            _crashed: false,
            _input: input,
        }
    }

    pub fn step(&mut self) -> Result<Option<i64>, ComputerError> {
        if self._crashed {
            return Err(ComputerError::Terminated);
        }

        loop {
            match self.process_next_instruction()? {
                Proceed => (),
                Output(value) => return Ok(Some(value)),
                Exit => return Ok(None),
            }
        }
    }
}

impl<I> Cpu<I> {
    pub fn patch_memory(&mut self, addr: usize, value: i64) {
        self.set_value(addr, value)
    }

    pub fn get_memory(&self) -> Vec<i64> {
        let mut result = Vec::new();
        for addr in 0.. {
            if let Some(value) = self._memory.get(&addr) {
                result.push(*value)
            } else {
                break;
            }
        }
        result
    }
}

impl<I> Cpu<I> {
    fn get_next_instruction(&self) -> Result<i64, ComputerError> {
        self._memory
            .get(&self._pointer)
            .copied()
            .ok_or(ComputerError::IllegalAddress(format!(
                "{} for next instruction",
                self._pointer
            )))
    }

    fn get_relative_address(&self, addr: usize, offset: i64) -> Result<usize, ComputerError> {
        let addr = self._memory.get(&addr).copied().unwrap_or_default() + offset;
        if addr < 0 {
            Err(ComputerError::IllegalAddress(format!(
                "{} for address with offset {}",
                addr, offset
            )))
        } else {
            Ok(addr as usize)
        }
    }

    fn get_addr(&self, addr: usize, mode: AddrMode) -> Result<usize, ComputerError> {
        match mode {
            AddrMode::Direct => Ok(addr),
            AddrMode::Absolut => self.get_relative_address(addr, 0),
            AddrMode::Relative => self.get_relative_address(addr, self._relative_base),
        }
    }

    fn get_value(&self, addr: usize, mode: AddrMode) -> Result<i64, ComputerError> {
        let addr = self.get_addr(addr, mode)?;
        let value = self._memory.get(&addr).copied().unwrap_or_default();
        Ok(value)
    }

    fn get_value_as_address(&self, addr: usize, mode: AddrMode) -> Result<usize, ComputerError> {
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

    fn set_value(&mut self, addr: usize, value: i64) {
        if let Some(cell) = self._memory.get_mut(&addr) {
            *cell = value;
        } else {
            self._memory.insert(addr, value);
        }
    }

    fn analyze_instruction(&self, instruction: i64) -> Result<(u8, AddrModes), ComputerError> {
        let op_code = (instruction % 100) as u8;
        Ok((op_code, AddrModes::new(instruction / 100)?))
    }

    fn add(&mut self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let op1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let op2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;

        self.set_value(addr, op1 + op2);
        self._pointer += 4;

        Ok(Proceed)
    }

    fn mul(&mut self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let op1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let op2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;

        self.set_value(addr, op1 * op2);
        self._pointer += 4;

        Ok(Proceed)
    }

    fn output(&mut self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let addr = self.get_addr(self._pointer + 1, modes.get(0))?;
        let value = self.get_value(addr, AddrMode::Direct)?;
        self._pointer += 2;

        Ok(Output(value))
    }

    fn jump_non_zero(&mut self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let cmp = self.get_value(self._pointer + 1, modes.get(0))?;
        self._pointer = if cmp != 0 {
            self.get_value_as_address(self._pointer + 2, modes.get(1))?
        } else {
            self._pointer + 3
        };

        Ok(Proceed)
    }

    fn jump_zero(&mut self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let cmp = self.get_value(self._pointer + 1, modes.get(0))?;
        self._pointer = if cmp == 0 {
            self.get_value_as_address(self._pointer + 2, modes.get(1))?
        } else {
            self._pointer + 3
        };

        Ok(Proceed)
    }

    fn less_than(&mut self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let cmp1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let cmp2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;
        self.set_value(addr, if cmp1 < cmp2 { 1 } else { 0 });
        self._pointer += 4;

        Ok(Proceed)
    }

    fn equals(&mut self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let cmp1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let cmp2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;
        self.set_value(addr, if cmp1 == cmp2 { 1 } else { 0 });
        self._pointer += 4;

        Ok(Proceed)
    }

    fn change_relative_base(
        &mut self,
        modes: &AddrModes,
    ) -> Result<OperationResult, ComputerError> {
        let offset = self.get_value(self._pointer + 1, modes.get(0))?;
        self._relative_base += offset;
        self._pointer += 2;

        Ok(Proceed)
    }

    fn exit(&mut self) -> Result<OperationResult, ComputerError> {
        self._pointer += 1;

        Ok(Exit)
    }
}

impl<I> Cpu<I>
where
    I: ComputerInput,
{
    fn process_next_instruction(&mut self) -> Result<OperationResult, ComputerError> {
        let instruction = self.get_next_instruction()?;
        let (opcode, modes) = self.analyze_instruction(instruction)?;

        match opcode {
            1 => self.add(&modes),
            2 => self.mul(&modes),
            3 => self.input(&modes),
            4 => self.output(&modes),
            5 => self.jump_non_zero(&modes),
            6 => self.jump_zero(&modes),
            7 => self.less_than(&modes),
            8 => self.equals(&modes),
            9 => self.change_relative_base(&modes),
            99 => self.exit(),

            _ => Err(ComputerError::UnknownInstruction(
                instruction,
                self._pointer,
            )),
        }
    }

    fn input(&mut self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        if let Some(input) = self._input.get_next_input() {
            let addr = self.get_addr(self._pointer + 1, modes.get(0))?;
            self.set_value(addr, input);
            self._pointer += 2;

            Ok(Proceed)
        } else {
            Err(ComputerError::InputEmpty)
        }
    }
}
