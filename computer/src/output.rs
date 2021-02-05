use std::cell::Cell;

use crate::{
    cpu::{CpuWrapper, StepResult},
    ComputerError,
};

#[derive(Debug)]
pub struct RawOutput<W>
where
    W: CpuWrapper,
{
    _cpu: W,
    _peek: Cell<Option<Option<i64>>>,
}

impl<W> RawOutput<W>
where
    W: CpuWrapper,
{
    pub fn new(cpu: W) -> RawOutput<W> {
        RawOutput {
            _cpu: cpu,
            _peek: Cell::new(None),
        }
    }

    pub fn step(&self) -> Result<StepResult, ComputerError> {
        self._peek.set(None);
        self._cpu.step()
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

    pub fn next(&self) -> Result<Option<i64>, ComputerError> {
        if let Some(peek) = self._peek.take() {
            return Ok(peek);
        }
        self._cpu.next()
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
