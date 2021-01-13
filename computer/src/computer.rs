use crate::{
    computer_error::ComputerError,
    input::{ComputerInput, ListInput},
};
use common::{as_long, read_single_line};
use modes::{AddrMode, AddrModes};
use std::{collections::HashMap, str::FromStr};

mod modes;

#[derive(Clone)]
pub struct Computer<I> {
    _memory: HashMap<usize, i64>,
    _pointer: usize,
    _terminated: bool,
    _input: I,
    _relative_base: i64,
    _peek: Option<Option<i64>>,
}

impl<I> Computer<I> {
    pub fn create(code: &[i64], input: I) -> Computer<I> {
        let mut _memory = HashMap::new();
        for (addr, inst) in code.iter().enumerate() {
            _memory.insert(addr, *inst);
        }

        Computer {
            _memory,
            _pointer: 0,
            _terminated: false,
            _input: input,
            _relative_base: 0,
            _peek: None,
        }
    }
}

enum InstResult {
    Proceed,
    Exit,
    Output(i64),
}

impl<I> Computer<I> {
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

    fn get_addr(&self, addr: usize, mode: AddrMode) -> Result<usize, ComputerError> {
        match mode {
            AddrMode::Absolut => {
                let addr = self._memory.get(&addr).copied().unwrap_or_default();
                if addr < 0 {
                    Err(ComputerError::IllegalAddress(format!(
                        "{} for getting address",
                        addr
                    )))
                } else {
                    Ok(addr as usize)
                }
            }

            AddrMode::Direct => Ok(addr),

            AddrMode::Relative => {
                let addr =
                    self._memory.get(&addr).copied().unwrap_or_default() + self._relative_base;
                if addr < 0 {
                    Err(ComputerError::IllegalAddress(format!(
                        "{} for relative adress base {}",
                        addr, self._relative_base
                    )))
                } else {
                    Ok(addr as usize)
                }
            }
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

    fn add(&mut self, modes: &AddrModes) -> Result<InstResult, ComputerError> {
        let op1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let op2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;

        self.set_value(addr, op1 + op2);
        self._pointer += 4;

        Ok(InstResult::Proceed)
    }

    fn mul(&mut self, modes: &AddrModes) -> Result<InstResult, ComputerError> {
        let op1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let op2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;

        self.set_value(addr, op1 * op2);
        self._pointer += 4;

        Ok(InstResult::Proceed)
    }

    fn output(&mut self, modes: &AddrModes) -> Result<InstResult, ComputerError> {
        let addr = self.get_addr(self._pointer + 1, modes.get(0))?;
        let value = self.get_value(addr, AddrMode::Direct)?;
        self._pointer += 2;
        Ok(InstResult::Output(value))
    }

    fn jump_non_zero(&mut self, modes: &AddrModes) -> Result<InstResult, ComputerError> {
        let cmp = self.get_value(self._pointer + 1, modes.get(0))?;
        self._pointer = if cmp != 0 {
            self.get_value_as_address(self._pointer + 2, modes.get(1))?
        } else {
            self._pointer + 3
        };

        Ok(InstResult::Proceed)
    }

    fn jump_zero(&mut self, modes: &AddrModes) -> Result<InstResult, ComputerError> {
        let cmp = self.get_value(self._pointer + 1, modes.get(0))?;
        self._pointer = if cmp == 0 {
            self.get_value_as_address(self._pointer + 2, modes.get(1))?
        } else {
            self._pointer + 3
        };

        Ok(InstResult::Proceed)
    }

    fn less_than(&mut self, modes: &AddrModes) -> Result<InstResult, ComputerError> {
        let cmp1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let cmp2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;
        self.set_value(addr, if cmp1 < cmp2 { 1 } else { 0 });
        self._pointer += 4;
        Ok(InstResult::Proceed)
    }

    fn equals(&mut self, modes: &AddrModes) -> Result<InstResult, ComputerError> {
        let cmp1 = self.get_value(self._pointer + 1, modes.get(0))?;
        let cmp2 = self.get_value(self._pointer + 2, modes.get(1))?;
        let addr = self.get_addr(self._pointer + 3, modes.get(2))?;
        self.set_value(addr, if cmp1 == cmp2 { 1 } else { 0 });
        self._pointer += 4;
        Ok(InstResult::Proceed)
    }

    fn change_relative_base(&mut self, modes: &AddrModes) -> Result<InstResult, ComputerError> {
        let offset = self.get_value(self._pointer + 1, modes.get(0))?;
        self._relative_base += offset;
        self._pointer += 2;
        Ok(InstResult::Proceed)
    }

    fn exit(&mut self) -> Result<InstResult, ComputerError> {
        self._pointer += 1;
        Ok(InstResult::Exit)
    }
}

impl<I> Computer<I>
where
    I: ComputerInput,
{
    pub fn provide_input(&mut self, value: i64) {
        self._input.provide_input(value)
    }

    pub fn write_input(&mut self, text: &str) -> Result<(), ComputerError> {
        for ch in text.chars() {
            let val = ch as i64;
            if 0 <= val && val <= 127 {
                self._input.provide_input(val);
            } else {
                return Err(ComputerError::NotValidAsciiChar(ch));
            }
        }
        self._input.provide_input(10);

        Ok(())
    }

    pub fn run(&mut self) -> Result<Vec<i64>, ComputerError> {
        self.collect()
    }

    pub fn read_output(&mut self) -> Result<Option<String>, ComputerError> {
        let mut result = String::new();

        if let Some(first) = self.peek()? {
            if 0 > first || first > 127 {
                return Ok(None);
            }
        }

        while let Some(code) = self.next() {
            let code = code?;
            if code == 10 {
                return Ok(Some(result));
            }
            if 0 <= code && code <= 127 {
                result.push((code as u8) as char);
            } else {
                return Err(ComputerError::NotValidAsciiInt(code));
            }
        }
        Ok(None)
    }

    pub fn do_steps(&mut self, num: usize) -> Result<Option<Vec<i64>>, ComputerError> {
        let result: Vec<i64> = self.take(num).collect::<Result<_, _>>()?;
        if result.len() != num {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }

    fn input(&mut self, modes: &AddrModes) -> Result<InstResult, ComputerError> {
        if let Some(input) = self._input.get_next_input() {
            let addr = self.get_addr(self._pointer + 1, modes.get(0))?;
            self.set_value(addr, input);
            self._pointer += 2;

            Ok(InstResult::Proceed)
        } else {
            Err(ComputerError::InputEmpty)
        }
    }

    pub fn peek(&mut self) -> Result<Option<i64>, ComputerError> {
        if let Some(result) = self._peek {
            Ok(result)
        } else if let Some(result) = self.next() {
            let result = result?;
            self._peek = Some(Some(result));
            Ok(Some(result))
        } else {
            self._peek = Some(None);
            Ok(None)
        }
    }
}

impl<I> Iterator for Computer<I>
where
    I: ComputerInput,
{
    type Item = Result<i64, ComputerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self._terminated {
            return None;
        }

        if let Some(result) = self._peek {
            self._peek = None;
            return result.map(|v| Ok(v));
        }

        while let Some(instruction) = self._memory.get(&self._pointer) {
            let result = match self.analyze_instruction(*instruction) {
                Ok((opcode, modes)) => match opcode {
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

                    _ => Err(ComputerError::UnknownOperation(opcode)),
                },
                Err(err) => Err(err),
            };

            match result {
                Ok(InstResult::Proceed) => (),
                Ok(InstResult::Output(value)) => return Some(Ok(value)),
                Ok(InstResult::Exit) => return None,

                Err(error) => {
                    self._terminated = true;
                    return Some(Err(error));
                }
            }
        }

        self._terminated = true;
        Some(Err(ComputerError::IllegalAddress(format!(
            "{} for next op code",
            self._pointer
        ))))
    }
}

impl Computer<ListInput> {
    pub fn new(code: &[i64]) -> Computer<ListInput> {
        Computer::create(code, ListInput::default())
    }

    pub fn from_file(module: &str, file: &str) -> Result<Computer<ListInput>, ComputerError> {
        let input = read_single_line(module, file)?;
        Computer::from_str(&input)
    }
}

impl FromStr for Computer<ListInput> {
    type Err = ComputerError;

    fn from_str(input: &str) -> Result<Computer<ListInput>, Self::Err> {
        let code: Vec<_> = input.split(",").map(as_long).collect::<Result<_, _>>()?;
        Ok(Computer::new(&code))
    }
}
