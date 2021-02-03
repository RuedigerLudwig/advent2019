use crate::{ComputerError, Output, TextInput};

#[derive(Debug)]
pub struct TextOutput {
    _output: Output<TextInput>,
}

impl TextOutput {
    pub fn new(output: Output<TextInput>) -> TextOutput {
        TextOutput { _output: output }
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
