use crate::{ComputerError, ListInput};

pub trait InputConverter {
    fn send_to(self, input: &mut ListInput) -> Result<(), ComputerError>;
}

impl InputConverter for String {
    fn send_to(self, input: &mut ListInput) -> Result<(), ComputerError> {
        self.as_str().send_to(input)
    }
}

impl InputConverter for &String {
    fn send_to(self, input: &mut ListInput) -> Result<(), ComputerError> {
        self.as_str().send_to(input)
    }
}

impl InputConverter for &str {
    fn send_to(self, input: &mut ListInput) -> Result<(), ComputerError> {
        for ch in self.chars() {
            let val = ch as i64;
            if 0 <= val && val <= 127 {
                input.provide_input(val);
            } else {
                return Err(ComputerError::NotValidAsciiChar(ch));
            }
        }
        input.provide_input(10);

        Ok(())
    }
}
