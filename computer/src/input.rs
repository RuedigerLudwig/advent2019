use common::pos::Pos;

use crate::ComputerError;

pub trait Input {
    fn get_data(&self) -> Result<Vec<i64>, ComputerError>;
}

impl Input for String {
    fn get_data(&self) -> Result<Vec<i64>, ComputerError> {
        self.as_str().get_data()
    }
}

impl Input for &String {
    fn get_data(&self) -> Result<Vec<i64>, ComputerError> {
        self.as_str().get_data()
    }
}

impl Input for &str {
    fn get_data(&self) -> Result<Vec<i64>, ComputerError> {
        let mut data = Vec::with_capacity(self.len() + 1);

        for ch in self.chars() {
            let val = ch as i64;
            if 0 <= val && val <= 127 {
                data.push(val);
            } else {
                return Err(ComputerError::NotValidAsciiChar(ch));
            }
        }
        data.push(10);

        Ok(data)
    }
}

impl Input for Pos<i64> {
    fn get_data(&self) -> Result<Vec<i64>, ComputerError> {
        Ok(vec![self.x(), self.y()])
    }
}
