use crate::{cpu::Cpu, Code, ComputerError, ComputerInput, StepResult};

#[derive(Debug)]
pub struct TextVM<'a> {
    cpu: Cpu<'a>,
    peek: Option<Option<i64>>,
}

impl<'a> TextVM<'a> {
    pub fn new(code: Code, input: impl ComputerInput + 'a) -> TextVM<'a> {
        let cpu = Cpu::new(code, input);
        TextVM { cpu, peek: None }
    }

    pub fn new_with_id(code: Code, input: impl ComputerInput + 'a, id: usize) -> TextVM<'a> {
        let mut cpu = Cpu::new(code, input);
        cpu.set_id(id);
        TextVM { cpu, peek: None }
    }

    pub fn read_line(&mut self) -> Result<Option<String>, ComputerError> {
        if let Some(peeked) = self.peek()? {
            match peeked {
                0..=127 => (),
                _ => return Ok(None),
            }
        }

        let mut result = String::new();
        while let Some(item) = self.next()? {
            match item {
                10 => return Ok(Some(result)),
                n @ 0..=127 => result.push((n as u8) as char),
                n => return Err(ComputerError::NotValidAsciiInt(n)),
            }
        }
        Ok(None)
    }

    fn peek(&mut self) -> Result<Option<i64>, ComputerError> {
        if let Some(peek) = self.peek {
            Ok(peek)
        } else {
            let peek = self.next()?;
            self.peek = Some(peek);
            Ok(peek)
        }
    }

    pub fn restart(&mut self) {
        self.cpu.restart()
    }

    pub fn set_debug_level(&mut self, debug_level: u8) {
        self.cpu.set_debug_level(debug_level)
    }

    pub fn patch_memory(&mut self, addr: usize, value: i64) {
        self.cpu.patch_memory(addr, value);
    }

    fn step(&mut self) -> Result<StepResult, ComputerError> {
        self.peek = None;
        self.cpu.step()
    }

    pub fn next(&mut self) -> Result<Option<i64>, ComputerError> {
        if let Some(peek) = self.peek.take() {
            return Ok(peek);
        }

        loop {
            match self.step()? {
                StepResult::Value(value) => return Ok(Some(value)),
                StepResult::Stop => return Ok(None),
                StepResult::Proceed => (),
                StepResult::WaitForInput => return Err(ComputerError::InputEmpty),
            }
        }
    }
}
