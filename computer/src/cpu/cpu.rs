use super::{
    common::analyze_instruction,
    debug_codes,
    debug_info::DebugInfo,
    modes::{AddrMode, AddrModes},
    operation_result::OperationResult,
    step_result::StepResult,
};
use crate::{Code, ComputerError, ComputerInput};
use std::collections::HashMap;

use OperationResult::*;

#[derive(Debug)]
pub struct Cpu<'a> {
    _code: Code,
    _memory: HashMap<usize, i64>,
    _offset: i64,
    _pointer: usize,
    _crashed: bool,
    _input: Box<dyn ComputerInput + 'a>,
    _debug_level: u8,
    _id: Option<usize>,
}

impl<'a> Cpu<'a> {
    pub fn new(code: Code, input: impl ComputerInput + 'a) -> Cpu<'a> {
        let memory = code.get();
        Cpu {
            _code: code,
            _memory: memory,
            _offset: 0,
            _pointer: 0,
            _crashed: false,
            _input: Box::new(input),
            _debug_level: debug_codes::NONE,
            _id: None,
        }
    }

    pub fn get_id(&self) -> Option<usize> {
        self._id
    }

    pub fn get_debug_level(&self) -> u8 {
        self._debug_level
    }

    pub fn get_offset(&self) -> i64 {
        self._offset
    }

    pub fn get_memory(&self) -> &HashMap<usize, i64> {
        &self._memory
    }

    pub fn restart(&mut self) {
        self._memory = self._code.get();
        self._offset = 0;
        self._pointer = 0;
        self._crashed = false;
    }

    pub fn set_id(&mut self, id: usize) {
        self._id = Some(id);
    }

    pub fn set_debug_level(&mut self, debug_level: u8) {
        self._debug_level = debug_level & debug_codes::ALL;
    }

    pub fn patch_memory(&mut self, addr: usize, value: i64) {
        self.set_value(addr, value)
    }

    pub fn get_linear_memory(&self) -> Vec<i64> {
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

    pub fn provide_input(&mut self, value: i64) {
        self._input.provide_input(value)
    }

    pub fn step(&mut self) -> Result<StepResult, ComputerError> {
        if self._crashed {
            return Err(ComputerError::Terminated);
        }

        match self.process_next_instruction() {
            Ok(Proceed { pointer }) => {
                self._pointer = pointer;
                Ok(StepResult::Proceed)
            }
            Ok(Offset { offset, pointer }) => {
                self._offset = offset;
                self._pointer = pointer;
                Ok(StepResult::Proceed)
            }
            Ok(Write {
                addr,
                value,
                pointer,
            }) => {
                self.set_value(addr, value);
                self._pointer = pointer;
                Ok(StepResult::Proceed)
            }
            Ok(Stop { pointer }) => {
                self._pointer = pointer;
                Ok(StepResult::Stop)
            }
            Ok(Output { value, pointer }) => {
                self._pointer = pointer;
                Ok(StepResult::Value(value))
            }
            Ok(WaitForInput) => Ok(StepResult::WaitForInput),
            Err(err) => {
                self._crashed = true;
                Err(err)
            }
        }
    }

    fn process_next_instruction(&mut self) -> Result<OperationResult, ComputerError> {
        let instruction = self.get_next_instruction()?;
        let (opcode, modes) = analyze_instruction(instruction)?;

        match opcode {
            1 => self.add(&modes),
            2 => self.mul(&modes),
            3 => self.input(&modes),
            4 => self.output(&modes),
            5 => self.jump_non_zero(&modes),
            6 => self.jump_zero(&modes),
            7 => self.less_than(&modes),
            8 => self.equals(&modes),
            9 => self.change_offset(&modes),
            99 => self.exit(&modes),

            _ => Err(ComputerError::UnknownInstruction(
                instruction,
                self._pointer,
            )),
        }
    }

    fn get_next_instruction(&self) -> Result<i64, ComputerError> {
        self._memory
            .get(&self._pointer)
            .copied()
            .ok_or(ComputerError::IllegalAddress(self._pointer as i64))
    }

    fn get_relative_address(&self, addr: usize, offset: i64) -> Result<usize, ComputerError> {
        let addr = self._memory.get(&addr).copied().unwrap_or_default() + offset;
        if addr < 0 {
            Err(ComputerError::IllegalAddress(addr))
        } else {
            Ok(addr as usize)
        }
    }

    fn get_addr(&self, addr: usize, mode: AddrMode) -> Result<usize, ComputerError> {
        match mode {
            AddrMode::Direct => Ok(addr),
            AddrMode::Absolut => self.get_relative_address(addr, 0),
            AddrMode::Relative => self.get_relative_address(addr, self._offset),
        }
    }

    fn get_value(&self, addr: usize, mode: AddrMode) -> Result<i64, ComputerError> {
        let addr = self.get_addr(addr, mode)?;
        let value = self._memory.get(&addr).copied().unwrap_or(0);
        Ok(value)
    }

    pub fn get_direct_value(&self, addr: usize) -> Result<i64, ComputerError> {
        if let Some(value) = self._memory.get(&addr) {
            Ok(*value)
        } else {
            Err(ComputerError::IllegalAddress(addr as i64))
        }
    }

    fn get_value_as_address(&self, addr: usize, mode: AddrMode) -> Result<usize, ComputerError> {
        let addr = self.get_value(addr, mode)?;
        if addr < 0 {
            Err(ComputerError::IllegalAddress(addr))
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

    fn add(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let op1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let op2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;
        let outcome = Write {
            addr,
            value: op1 + op2,
            pointer: self._pointer + 4,
        };

        if self._debug_level != debug_codes::NONE {
            DebugInfo::new(
                self,
                "ADD",
                (self._pointer, self._pointer + 4),
                modes,
                &format!("{} + {} -> {} to [{}]", op1, op2, op1 + op2, addr),
                outcome,
            )
            .add_params(&vec![op1, op2])
            .add_write(op1 + op2, addr)
            .print()?;
        }

        Ok(outcome)
    }

    fn mul(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let op1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let op2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;
        let outcome = Write {
            addr,
            value: op1 * op2,
            pointer: self._pointer + 4,
        };

        if self._debug_level != debug_codes::NONE {
            DebugInfo::new(
                self,
                "MUL",
                (self._pointer, self._pointer + 4),
                modes,
                &format!("{} * {} -> {} to [{}]", op1, op2, op1 * op2, addr),
                outcome,
            )
            .add_params(&vec![op1, op2])
            .add_write(op1 * op2, addr)
            .print()?;
        }

        Ok(outcome)
    }

    fn input(&mut self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        if let Some(value) = self._input.get_next_input() {
            let addr = self.get_addr(self._pointer + 1, modes.get(0))?;
            let outcome = Write {
                addr,
                value,
                pointer: self._pointer + 2,
            };

            if self._debug_level != debug_codes::NONE {
                DebugInfo::new(
                    self,
                    "INP",
                    (self._pointer, self._pointer + 2),
                    modes,
                    &format!("Input {} to [{}]", value, addr),
                    outcome,
                )
                .add_write(value, addr)
                .print()?;
            }

            Ok(outcome)
        } else {
            Ok(WaitForInput)
        }
    }

    fn output(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let addr = self.get_addr(self._pointer + 1, modes.get(0))?;
        let value = self.get_value(addr, AddrMode::Direct)?;
        let outcome = Output {
            value,
            pointer: self._pointer + 2,
        };

        if self._debug_level != debug_codes::NONE {
            DebugInfo::new(
                self,
                "OUT",
                (self._pointer, self._pointer + 2),
                modes,
                &format!("Output {} from [{}]", value, addr),
                outcome,
            )
            .add_params(&vec![addr as i64])
            .print()?;
        }
        Ok(outcome)
    }

    fn jump_non_zero(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let cmp = self.get_value(self._pointer + 1, modes.get(0))?;
        let to = self.get_value_as_address(self._pointer + 2, modes.get(1))?;
        let outcome = Proceed {
            pointer: if cmp != 0 { to } else { self._pointer + 3 },
        };

        if self._debug_level != debug_codes::NONE {
            let info_text = if cmp != 0 {
                format!("{} != 0 -> Pointer ({})", cmp, to)
            } else {
                format!("0 == 0 -> Pointer ({})", self._pointer + 3)
            };

            DebugInfo::new(
                self,
                "JNZ",
                (self._pointer, self._pointer + 3),
                modes,
                &info_text,
                outcome,
            )
            .add_params(&vec![cmp])
            .print()?;
        }

        Ok(outcome)
    }

    fn jump_zero(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let cmp = self.get_value(self._pointer + 1, modes.get(0))?;
        let to = self.get_value_as_address(self._pointer + 2, modes.get(1))?;
        let outcome = Proceed {
            pointer: if cmp == 0 { to } else { self._pointer + 3 },
        };

        if self._debug_level != debug_codes::NONE {
            let info_text = if cmp == 0 {
                format!("0 == 0 -> Pointer ({})", to)
            } else {
                format!("{} != 0 -> Pointer ({})", cmp, self._pointer + 3)
            };

            DebugInfo::new(
                self,
                "JZ",
                (self._pointer, self._pointer + 3),
                modes,
                &info_text,
                outcome,
            )
            .add_params(&vec![cmp])
            .print()?;
        }

        Ok(outcome)
    }

    fn less_than(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let cmp1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let cmp2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;
        let value = if cmp1 < cmp2 { 1 } else { 0 };
        let outcome = Write {
            addr,
            value,
            pointer: self._pointer + 4,
        };

        if self._debug_level != debug_codes::NONE {
            let info_text = if cmp1 < cmp2 {
                format!("{} < {} -> {} to [{}]", cmp1, cmp2, value, addr)
            } else {
                format!("{} >= {} -> {} to [{}]", cmp1, cmp2, value, addr)
            };
            DebugInfo::new(
                self,
                "LT",
                (self._pointer, self._pointer + 4),
                modes,
                &info_text,
                outcome,
            )
            .add_params(&vec![cmp1, cmp2])
            .add_write(value, addr)
            .print()?;
        }

        Ok(outcome)
    }

    fn equals(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let cmp1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let cmp2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;
        let value = if cmp1 == cmp2 { 1 } else { 0 };
        let outcome = Write {
            addr,
            value,
            pointer: self._pointer + 4,
        };

        if self._debug_level != debug_codes::NONE {
            let info_text = if cmp1 == cmp2 {
                format!("{} == {} -> {} to [{}]", cmp1, cmp2, value, addr)
            } else {
                format!("{} != {} -> {} to [{}]", cmp1, cmp2, value, addr)
            };
            DebugInfo::new(
                self,
                "EQ",
                (self._pointer, self._pointer + 4),
                modes,
                &info_text,
                outcome,
            )
            .add_params(&vec![cmp1, cmp2])
            .add_write(value, addr)
            .print()?;
        }

        Ok(outcome)
    }

    fn change_offset(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let offset = self.get_value(self._pointer + 1, modes.get(0))?;
        let outcome = Offset {
            offset: self._offset + offset,
            pointer: self._pointer + 2,
        };

        if self._debug_level != debug_codes::NONE {
            DebugInfo::new(
                self,
                "OFF",
                (self._pointer, self._pointer + 2),
                modes,
                &format!("Offset + {} => {{{}}}", offset, self._offset),
                outcome,
            )
            .add_params(&vec![offset])
            .print()?;
        }

        Ok(outcome)
    }

    fn exit(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let outcome = Stop {
            pointer: self._pointer + 1,
        };
        if self._debug_level != debug_codes::NONE {
            DebugInfo::new(
                self,
                "STP",
                (self._pointer, self._pointer + 1),
                modes,
                "Stop",
                outcome,
            )
            .print()?;
        }

        Ok(outcome)
    }
}
