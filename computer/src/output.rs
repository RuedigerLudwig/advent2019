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

    pub fn get_all(&mut self) -> Result<Vec<i64>, ComputerError> {
        self.collect::<Result<Vec<_>, _>>()
    }

    pub fn take_exactly(&mut self, count: usize) -> Result<Option<Vec<i64>>, ComputerError> {
        let result = self.take(count).collect::<Result<Vec<_>, _>>()?;
        if result.len() != count {
            Ok(None)
        } else {
            Ok(Some(result))
        }
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
