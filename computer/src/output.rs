use std::cell::Cell;

use crate::{cpu::StepResult, input::ComputerInput, vm::CpuWrapper, ComputerError};

#[derive(Debug)]
pub struct Output<I> {
    _cpu: CpuWrapper<I>,
    _peek: Cell<Option<Option<i64>>>,
}

impl<'a, I> Output<I>
where
    I: ComputerInput,
{
    pub fn new(cpu: CpuWrapper<I>) -> Output<I> {
        Output {
            _cpu: cpu,
            _peek: Cell::new(None),
        }
    }

    pub fn get_all(&self) -> Result<Vec<i64>, ComputerError> {
        let mut result = Vec::new();
        while let Some(compute) = self.next()? {
            result.push(compute);
        }
        Ok(result)
    }

    pub fn take_exactly(&self, count: usize) -> Result<Option<Vec<i64>>, ComputerError> {
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

    pub fn step(&self) -> Result<StepResult, ComputerError> {
        self._peek.set(None);
        self._cpu.step()
    }

    pub fn next(&self) -> Result<Option<i64>, ComputerError> {
        if let Some(peek) = self._peek.take() {
            return Ok(peek);
        }
        loop {
            match self.step()? {
                StepResult::Value(value) => return Ok(Some(value)),
                StepResult::Stop => return Ok(None),
                StepResult::Proceed => (),
                StepResult::Blocked => (),
            }
        }
    }

    pub fn peek(&self) -> Result<Option<i64>, ComputerError> {
        if let Some(peek) = self._peek.get() {
            Ok(peek)
        } else {
            let peek = self.next()?;
            self._peek.set(Some(peek));
            Ok(peek)
        }
    }
}
/*
impl<I> Iterator for Output<I>
where
I: ComputerInput,
{
    type Item = Result<i64, ComputerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self._cpu.next_output().transpose()
    }
}

*/
