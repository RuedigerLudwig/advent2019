use crate::{input::ComputerInput, vm::CpuWrapper, ComputerError};

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
        while let Some(compute) = self._cpu.step()? {
            result.push(compute);
        }
        Ok(result)
    }

    pub fn take_exactly(&self, count: usize) -> Result<Option<Vec<i64>>, ComputerError> {
        let mut result = Vec::new();

        for _ in 0..count {
            if let Some(compute) = self._cpu.step()? {
                result.push(compute)
            } else {
                return Ok(None);
            }
        }

        Ok(Some(result))
    }
}

impl<I> Iterator for Output<I>
where
    I: ComputerInput,
{
    type Item = Result<i64, ComputerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self._cpu.step() {
            Ok(Some(value)) => Some(Ok(value)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
