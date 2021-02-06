use crate::{cpu::Cpu, input::ComputerInput, Code, ComputerError, StepResult};

#[derive(Debug)]
pub struct VirtualMachine<'a> {
    _cpu: Cpu<'a>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(code: Code, input: impl ComputerInput + 'a) -> VirtualMachine<'a> {
        let cpu = Cpu::new(code, input);
        VirtualMachine { _cpu: cpu }
    }

    pub fn new_with_id(
        code: Code,
        input: impl ComputerInput + 'a,
        id: usize,
    ) -> VirtualMachine<'a> {
        let mut cpu = Cpu::new(code, input);
        cpu.set_id(id);
        VirtualMachine { _cpu: cpu }
    }

    pub fn restart(&mut self) {
        self._cpu.restart()
    }

    pub fn set_debug_level(&mut self, debug_level: u8) {
        self._cpu.set_debug_level(debug_level);
    }

    pub fn patch_memory(&mut self, addr: usize, value: i64) {
        self._cpu.patch_memory(addr, value);
    }

    pub fn get_memory(&self) -> Vec<i64> {
        (0..)
            .map_while(|addr| self._cpu.get_direct_value(addr).ok())
            .collect()
    }

    pub fn step(&mut self) -> Result<StepResult, ComputerError> {
        self._cpu.step()
    }

    pub fn next(&mut self) -> Result<Option<i64>, ComputerError> {
        loop {
            match self.step()? {
                StepResult::Value(value) => return Ok(Some(value)),
                StepResult::Stop => return Ok(None),
                StepResult::Proceed => (),
                StepResult::WaitForInput => return Err(ComputerError::InputEmpty),
            }
        }
    }

    pub fn get_all(&mut self) -> Result<Vec<i64>, ComputerError> {
        let mut result = Vec::new();
        while let Some(compute) = self.next()? {
            result.push(compute);
        }
        Ok(result)
    }

    pub fn take_exactly(&mut self, count: usize) -> Result<Option<Vec<i64>>, ComputerError> {
        let mut result = Vec::new();

        for _ in 0..count {
            if let Some(compute) = self.next()? {
                result.push(compute)
            } else {
                return Ok(None);
            }
        }

        Ok(Some(result))
    }
}
