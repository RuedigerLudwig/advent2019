use crate::{cpu::StepResult, input::ComputerInput, vm::CpuWrapper, ComputerError};

#[derive(Debug)]
pub struct Output<I> {
    _cpu: CpuWrapper<I>,
}

impl<'a, I> Output<I>
where
    I: ComputerInput,
{
    pub fn new(cpu: CpuWrapper<I>) -> Output<I> {
        Output { _cpu: cpu }
    }

    pub fn get_all(&self) -> Result<Vec<i64>, ComputerError> {
        let mut result = Vec::new();
        while let Some(compute) = self._cpu.next_output()? {
            result.push(compute);
        }
        Ok(result)
    }

    pub fn take_exactly(&self, count: usize) -> Result<Option<Vec<i64>>, ComputerError> {
        let mut result = Vec::new();

        for _ in 0..count {
            if let Some(compute) = self._cpu.next_output()? {
                result.push(compute)
            } else {
                return Ok(None);
            }
        }

        Ok(Some(result))
    }

    pub fn step(&self) -> Result<StepResult, ComputerError> {
        self._cpu.step()
    }
}

impl<I> Iterator for Output<I>
where
    I: ComputerInput,
{
    type Item = Result<i64, ComputerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self._cpu.next_output() {
            Ok(Some(value)) => Some(Ok(value)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
