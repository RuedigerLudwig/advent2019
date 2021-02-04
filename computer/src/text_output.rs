use crate::{
    cpu_wrapper::{CpuWrapper, MultiThreadWrapper, SingleThreadWrapper},
    output::RawOutput,
    ComputerError,
};

pub type STTextOutput<'a> = RawTextOutput<SingleThreadWrapper<'a>>;
pub type MTTextOutput<'a> = RawTextOutput<MultiThreadWrapper<'a>>;

#[derive(Debug)]
pub struct RawTextOutput<W>
where
    W: CpuWrapper,
{
    _output: RawOutput<W>,
}

impl<'a, W> RawTextOutput<W>
where
    W: CpuWrapper,
{
    pub fn new(output: RawOutput<W>) -> RawTextOutput<W> {
        RawTextOutput { _output: output }
    }

    pub fn read_line(&self) -> Result<Option<String>, ComputerError> {
        if let Some(peeked) = self._output.peek()? {
            match peeked {
                0..=127 => (),
                _ => return Ok(None),
            }
        }

        let mut result = String::new();
        while let Some(item) = self._output.next()? {
            match item {
                10 => return Ok(Some(result)),
                n @ 0..=127 => result.push((n as u8) as char),
                n => return Err(ComputerError::NotValidAsciiInt(n)),
            }
        }
        Ok(None)
    }

    pub fn int_value(&self) -> Result<Option<i64>, ComputerError> {
        if let Some(result) = self._output.next()? {
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}
