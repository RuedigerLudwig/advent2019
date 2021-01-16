use std::iter::Peekable;

use crate::{ComputerError, Output, TextInput};

pub struct TextOutput {
    output: Peekable<Output<TextInput>>,
}

impl TextOutput {
    pub fn new(output: Output<TextInput>) -> TextOutput {
        TextOutput {
            output: output.peekable(),
        }
    }

    pub fn read_line(&mut self) -> Result<Option<String>, ComputerError> {
        match self.output.peek() {
            Some(Ok(n)) => match n {
                0..=127 => (),
                _ => return Ok(None),
            },
            Some(Err(err)) => return Err(err.clone()),
            None => (),
        }

        let mut result = String::new();
        while let Some(item) = self.output.next() {
            match item? {
                10 => return Ok(Some(result)),
                n @ 0..=127 => result.push((n as u8) as char),
                n => return Err(ComputerError::NotValidAsciiInt(n)),
            }
        }
        Ok(None)
    }
}

impl Iterator for TextOutput {
    type Item = Result<i64, ComputerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.output.next()
    }
}
