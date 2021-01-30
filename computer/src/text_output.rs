use std::{cell::RefCell, iter::Peekable};

use crate::{ComputerError, Output, TextInput};

pub struct TextOutput {
    _output: RefCell<Peekable<Output<TextInput>>>,
}

impl TextOutput {
    pub fn new(output: Output<TextInput>) -> TextOutput {
        TextOutput {
            _output: RefCell::new(output.peekable()),
        }
    }

    pub fn read_line(&self) -> Result<Option<String>, ComputerError> {
        let mut output = self._output.borrow_mut();

        match output.peek() {
            Some(Ok(n)) => match n {
                0..=127 => (),
                _ => return Ok(None),
            },
            Some(Err(err)) => return Err(err.clone()),
            None => (),
        }

        let mut result = String::new();
        while let Some(item) = output.next() {
            match item? {
                10 => return Ok(Some(result)),
                n @ 0..=127 => result.push((n as u8) as char),
                n => return Err(ComputerError::NotValidAsciiInt(n)),
            }
        }
        Ok(None)
    }

    pub fn int_value(&self) -> Result<Option<i64>, ComputerError> {
        let mut output = self._output.borrow_mut();
        if let Some(result) = output.next() {
            Ok(Some(result?))
        } else {
            Ok(None)
        }
    }
}
