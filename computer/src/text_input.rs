use crate::{input::ComputerInput, ComputerError, ListInput};

#[derive(Debug, Clone)]
pub struct TextInput {
    _input: ListInput,
}

impl TextInput {
    pub fn new() -> TextInput {
        TextInput {
            _input: ListInput::new(),
        }
    }

    pub fn write_input(&self, text: &str) -> Result<(), ComputerError> {
        for ch in text.chars() {
            let val = ch as i64;
            if 0 <= val && val <= 127 {
                self._input.provide_input(val);
            } else {
                return Err(ComputerError::NotValidAsciiChar(ch));
            }
        }
        self._input.provide_input(10);

        Ok(())
    }
}

impl ComputerInput for TextInput {
    fn get_next_input(&self) -> Option<i64> {
        self._input.get_next_input()
    }
}
