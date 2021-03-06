use super::super::{Code, ComputerError, ComputerInput};
use super::{
    debug_codes,
    debug_info::DebugInfo,
    modes::{AddrMode, AddrModes},
    operation_result::OperationResult,
    step_result::StepResult,
};
use std::collections::HashMap;

use OperationResult::*;

#[derive(Debug)]
pub struct Cpu<'a> {
    code: Code,
    memory: HashMap<usize, i64>,
    offset: i64,
    pointer: usize,
    crashed: bool,
    input: Box<dyn ComputerInput + 'a>,
    debug_level: u8,
    id: Option<usize>,
}

impl<'a> Cpu<'a> {
    pub fn new(code: Code, input: impl ComputerInput + 'a) -> Cpu<'a> {
        let memory = code.get();
        Cpu {
            code,
            memory,
            offset: 0,
            pointer: 0,
            crashed: false,
            input: Box::new(input),
            debug_level: debug_codes::NONE,
            id: None,
        }
    }

    pub fn get_id(&self) -> Option<usize> {
        self.id
    }

    pub fn get_debug_level(&self) -> u8 {
        self.debug_level
    }

    pub fn get_offset(&self) -> i64 {
        self.offset
    }

    pub fn get_memory(&self) -> &HashMap<usize, i64> {
        &self.memory
    }

    pub fn restart(&mut self) {
        self.memory = self.code.get();
        self.offset = 0;
        self.pointer = 0;
        self.crashed = false;
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = Some(id);
    }

    pub fn set_debug_level(&mut self, debug_level: u8) {
        self.debug_level = debug_level & debug_codes::ALL;
    }

    pub fn patch_memory(&mut self, addr: usize, value: i64) {
        self.memory.insert(addr, value);
    }

    pub fn step(&mut self) -> Result<StepResult, ComputerError> {
        if self.crashed {
            return Err(ComputerError::Terminated);
        }

        match self.process_next_instruction() {
            Ok(Proceed { pointer }) => {
                self.pointer = pointer;
                Ok(StepResult::Proceed)
            }

            Ok(Offset { offset, pointer }) => {
                self.offset = offset;
                self.pointer = pointer;
                Ok(StepResult::Proceed)
            }

            Ok(Write {
                addr,
                value,
                pointer,
            }) => {
                self.memory.insert(addr, value);
                self.pointer = pointer;
                Ok(StepResult::Proceed)
            }

            Ok(Stop { pointer }) => {
                self.pointer = pointer;
                Ok(StepResult::Stop)
            }

            Ok(Output { value, pointer }) => {
                self.pointer = pointer;
                Ok(StepResult::Value(value))
            }

            Ok(WaitForInput) => Ok(StepResult::WaitForInput),

            Err(err) => {
                self.crashed = true;
                Err(err)
            }
        }
    }

    fn process_next_instruction(&mut self) -> Result<OperationResult, ComputerError> {
        let instruction = self.get_next_instruction()?;
        let (opcode, modes) = AddrModes::analyze_instruction(instruction)?;

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

            _ => Err(ComputerError::UnknownInstruction(instruction, self.pointer)),
        }
    }

    fn get_next_instruction(&self) -> Result<i64, ComputerError> {
        self.memory
            .get(&self.pointer)
            .copied()
            .ok_or(ComputerError::IllegalAddress(self.pointer as i64))
    }

    fn get_relative_address(&self, addr: usize, offset: i64) -> Result<usize, ComputerError> {
        let addr = self.memory.get(&addr).copied().unwrap_or_default() + offset;
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
            AddrMode::Relative => self.get_relative_address(addr, self.offset),
        }
    }

    fn get_value(&self, addr: usize, mode: AddrMode) -> Result<i64, ComputerError> {
        let addr = self.get_addr(addr, mode)?;
        let value = self.memory.get(&addr).copied().unwrap_or(0);
        Ok(value)
    }

    pub fn get_direct_value(&self, addr: usize) -> Result<i64, ComputerError> {
        self.memory
            .get(&addr)
            .copied()
            .ok_or(ComputerError::IllegalAddress(addr as i64))
    }

    fn get_value_as_address(&self, addr: usize, mode: AddrMode) -> Result<usize, ComputerError> {
        let addr = self.get_value(addr, mode)?;
        if addr < 0 {
            Err(ComputerError::IllegalAddress(addr))
        } else {
            Ok(addr as usize)
        }
    }

    fn add(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let op1 = self.get_value(self.pointer + 1, modes[0])?;
        let op2 = self.get_value(self.pointer + 2, modes[1])?;
        let addr = self.get_addr(self.pointer + 3, modes[2])?;
        let outcome = Write {
            addr,
            value: op1 + op2,
            pointer: self.pointer + 4,
        };

        if self.debug_level != debug_codes::NONE {
            DebugInfo::new(
                self,
                "ADD",
                (self.pointer, self.pointer + 4),
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
        let op1 = self.get_value(self.pointer + 1, modes[0])?;
        let op2 = self.get_value(self.pointer + 2, modes[1])?;
        let addr = self.get_addr(self.pointer + 3, modes[2])?;
        let outcome = Write {
            addr,
            value: op1 * op2,
            pointer: self.pointer + 4,
        };

        if self.debug_level != debug_codes::NONE {
            DebugInfo::new(
                self,
                "MUL",
                (self.pointer, self.pointer + 4),
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
        if let Some(value) = self.input.get_next_input() {
            let addr = self.get_addr(self.pointer + 1, modes[0])?;
            let outcome = Write {
                addr,
                value,
                pointer: self.pointer + 2,
            };

            if self.debug_level != debug_codes::NONE {
                DebugInfo::new(
                    self,
                    "INP",
                    (self.pointer, self.pointer + 2),
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
        let addr = self.get_addr(self.pointer + 1, modes[0])?;
        let value = self.get_value(addr, AddrMode::Direct)?;
        let outcome = Output {
            value,
            pointer: self.pointer + 2,
        };

        if self.debug_level != debug_codes::NONE {
            DebugInfo::new(
                self,
                "OUT",
                (self.pointer, self.pointer + 2),
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
        let cmp = self.get_value(self.pointer + 1, modes[0])?;
        let to = self.get_value_as_address(self.pointer + 2, modes[1])?;
        let outcome = Proceed {
            pointer: if cmp != 0 { to } else { self.pointer + 3 },
        };

        if self.debug_level != debug_codes::NONE {
            let info_text = if cmp != 0 {
                format!("{} != 0 -> Pointer ({})", cmp, to)
            } else {
                format!("0 == 0 -> Pointer ({})", self.pointer + 3)
            };

            DebugInfo::new(
                self,
                "JNZ",
                (self.pointer, self.pointer + 3),
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
        let cmp = self.get_value(self.pointer + 1, modes[0])?;
        let to = self.get_value_as_address(self.pointer + 2, modes[1])?;
        let outcome = Proceed {
            pointer: if cmp == 0 { to } else { self.pointer + 3 },
        };

        if self.debug_level != debug_codes::NONE {
            let info_text = if cmp == 0 {
                format!("0 == 0 -> Pointer ({})", to)
            } else {
                format!("{} != 0 -> Pointer ({})", cmp, self.pointer + 3)
            };

            DebugInfo::new(
                self,
                "JZ",
                (self.pointer, self.pointer + 3),
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
        let cmp1 = self.get_value(self.pointer + 1, modes[0])?;
        let cmp2 = self.get_value(self.pointer + 2, modes[1])?;
        let addr = self.get_addr(self.pointer + 3, modes[2])?;
        let value = if cmp1 < cmp2 { 1 } else { 0 };
        let outcome = Write {
            addr,
            value,
            pointer: self.pointer + 4,
        };

        if self.debug_level != debug_codes::NONE {
            let info_text = if cmp1 < cmp2 {
                format!("{} < {} -> {} to [{}]", cmp1, cmp2, value, addr)
            } else {
                format!("{} >= {} -> {} to [{}]", cmp1, cmp2, value, addr)
            };
            DebugInfo::new(
                self,
                "LT",
                (self.pointer, self.pointer + 4),
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
        let cmp1 = self.get_value(self.pointer + 1, modes[0])?;
        let cmp2 = self.get_value(self.pointer + 2, modes[1])?;
        let addr = self.get_addr(self.pointer + 3, modes[2])?;
        let value = if cmp1 == cmp2 { 1 } else { 0 };
        let outcome = Write {
            addr,
            value,
            pointer: self.pointer + 4,
        };

        if self.debug_level != debug_codes::NONE {
            let info_text = if cmp1 == cmp2 {
                format!("{} == {} -> {} to [{}]", cmp1, cmp2, value, addr)
            } else {
                format!("{} != {} -> {} to [{}]", cmp1, cmp2, value, addr)
            };
            DebugInfo::new(
                self,
                "EQ",
                (self.pointer, self.pointer + 4),
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
        let offset = self.get_value(self.pointer + 1, modes[0])?;
        let outcome = Offset {
            offset: self.offset + offset,
            pointer: self.pointer + 2,
        };

        if self.debug_level != debug_codes::NONE {
            DebugInfo::new(
                self,
                "OFF",
                (self.pointer, self.pointer + 2),
                modes,
                &format!("Offset + {} => {{{}}}", offset, self.offset),
                outcome,
            )
            .add_params(&vec![offset])
            .print()?;
        }

        Ok(outcome)
    }

    fn exit(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let outcome = Stop {
            pointer: self.pointer + 1,
        };
        if self.debug_level != debug_codes::NONE {
            DebugInfo::new(
                self,
                "STP",
                (self.pointer, self.pointer + 1),
                modes,
                "Stop",
                outcome,
            )
            .print()?;
        }

        Ok(outcome)
    }
}
