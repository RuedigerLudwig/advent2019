use std::{collections::HashMap, fmt::Display};

use crate::{
    common::{analyze_instruction, disassemble},
    error::ComputerError,
    input::{ComputerInput, Input},
    modes::{AddrMode, AddrModes},
};
pub mod debug {
    pub const NONE: u8 = 0b0000u8;
    pub const HEAD: u8 = 0b0001u8;
    pub const TEXT_INFO: u8 = 0b0010u8;
    pub const FULL_INFO: u8 = 0b0100u8;
    pub const DISASSEMBLE: u8 = 0b1000u8;
    pub const ALL: u8 = 0b1111u8;
}

struct DebugInfo<'a, I> {
    cpu: &'a Cpu<I>,
    id: String,
    offset: i64,

    name: &'a str,
    range: (usize, usize),
    modes: &'a AddrModes,
    info_text: &'a str,
    outcome: OperationResult,

    processed_params: Option<&'a [i64]>,

    write: Option<(i64, usize)>,
}

impl<'a, I> DebugInfo<'a, I> {
    pub fn new(
        cpu: &'a Cpu<I>,
        name: &'a str,
        range: (usize, usize),
        modes: &'a AddrModes,
        info_text: &'a str,
        outcome: OperationResult,
    ) -> DebugInfo<'a, I> {
        DebugInfo {
            cpu,
            offset: cpu._offset,
            id: cpu._id.clone(),
            name,
            range,
            modes,
            info_text,
            outcome,
            processed_params: None,
            write: None,
        }
    }

    pub fn add_params(mut self, processed_params: &'a [i64]) -> Self {
        self.processed_params = Some(processed_params);
        self
    }

    pub fn add_write(mut self, write_result: i64, write_addr: usize) -> Self {
        self.write = Some((write_result, write_addr));
        self
    }

    fn print(&self) -> Result<(), ComputerError> {
        if self.cpu._debug_level & debug::HEAD != 0 {
            println!(
                "({}-{}) | {{{}}} | \"{}\"",
                self.range.0,
                self.range.1 - 1,
                self.offset,
                self.id
            );
        }

        if self.cpu._debug_level & debug::DISASSEMBLE != 0 {
            let (output, _) = disassemble(&self.cpu._memory, self.range.0)?;
            println!("    {}", output);
        }

        if self.cpu._debug_level & debug::FULL_INFO != 0 {
            println!("    Info: {:3}", self.name);
            println!(
                "          Inst:    {}",
                self.modes
                    .get(0)
                    .format(self.cpu.get_direct_value(self.range.0)?, 4)
            );
            print!("          Raw:     ");
            for ad in self.range.0 + 1..self.range.1 {
                let mode = self.modes.get(ad - self.range.0);
                print!("{} ", mode.format(self.cpu.get_direct_value(ad)?, 4));
            }
            println!();
            if let Some(processed_params) = self.processed_params {
                println!("          Params:  {}", join(processed_params, ", "));
            }
            if let Some((result, addr)) = self.write {
                println!("          Write    {} to {}", result, addr);
            }
            println!("          Result:  {}", self.outcome);
        }

        if self.cpu._debug_level & debug::TEXT_INFO != 0 {
            println!("    {}", self.info_text);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Cpu<I> {
    _memory: HashMap<usize, i64>,
    _offset: i64,
    _pointer: usize,
    _crashed: bool,
    _input: I,
    _debug_level: u8,
    _id: String,
}

#[derive(Debug, Clone, Copy)]
enum OperationResult {
    Proceed {
        pointer: usize,
    },
    Offset {
        offset: i64,
        pointer: usize,
    },
    Write {
        addr: usize,
        value: i64,
        pointer: usize,
    },
    Output {
        value: i64,
        pointer: usize,
    },
    Stop {
        pointer: usize,
    },
    WaitForInput,
}

impl Display for OperationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Write {
                addr,
                value,
                pointer,
            } => write!(
                f,
                "Write {} to [{}] and proceed with ({})",
                value, addr, pointer
            ),
            Proceed { pointer } => write!(f, "Proceed with ({})", pointer),
            Stop { pointer } => write!(f, "Stop at ({})", pointer),
            Output { value, pointer } => {
                write!(f, "Output {} and proceed with ({})", value, pointer)
            }
            Offset { offset, pointer } => {
                write!(f, "Offset to {{{}}} and proceed with ({})", offset, pointer)
            }
            WaitForInput => write!(f, "Waiting for Input"),
        }
    }
}

use common::join;
use OperationResult::*;

impl<I> Cpu<I> {
    pub fn new(code: HashMap<usize, i64>, input: I) -> Cpu<I> {
        Cpu {
            _memory: code,
            _offset: 0,
            _pointer: 0,
            _crashed: false,
            _input: input,
            _debug_level: debug::NONE,
            _id: "".to_owned(),
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self._id = id.to_owned();
    }

    pub fn set_debug_level(&mut self, debug_level: u8) {
        self._debug_level = debug_level & debug::ALL;
    }

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

    fn get_direct_value(&self, addr: usize) -> Result<i64, ComputerError> {
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

        if self._debug_level != debug::NONE {
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

        if self._debug_level != debug::NONE {
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

    fn output(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        let addr = self.get_addr(self._pointer + 1, modes.get(0))?;
        let value = self.get_value(addr, AddrMode::Direct)?;
        let outcome = Output {
            value,
            pointer: self._pointer + 2,
        };

        if self._debug_level != debug::NONE {
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

        if self._debug_level != debug::NONE {
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

        if self._debug_level != debug::NONE {
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

        if self._debug_level != debug::NONE {
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

        if self._debug_level != debug::NONE {
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

        if self._debug_level != debug::NONE {
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
        if self._debug_level != debug::NONE {
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

#[derive(Debug, Copy, Clone)]
pub enum StepResult {
    Value(i64),
    Stop,
    Proceed,
    Blocked,
}

impl<I> Cpu<I>
where
    I: ComputerInput,
{
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
            Ok(WaitForInput) => Ok(StepResult::Blocked),
            Err(err) => {
                self._crashed = true;
                Err(err)
            }
        }
    }

    fn process_next_instruction(&self) -> Result<OperationResult, ComputerError> {
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

    fn input(&self, modes: &AddrModes) -> Result<OperationResult, ComputerError> {
        match self._input.get_next_input() {
            Input::Value(value) => {
                let addr = self.get_addr(self._pointer + 1, modes.get(0))?;
                let outcome = Write {
                    addr,
                    value,
                    pointer: self._pointer + 2,
                };

                if self._debug_level != debug::NONE {
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
            }
            Input::NoMoreValues => Err(ComputerError::InputEmpty),
            Input::WaitForValue => Ok(WaitForInput),
        }
    }
}
