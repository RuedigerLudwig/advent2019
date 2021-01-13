use computer::Computer;

use crate::exterior_error::ExteriorError;

pub trait ExteriorInterface {
    fn get_picture(&mut self) -> Result<String, ExteriorError>;
    fn send_data(&mut self, data: &[String]) -> Result<i64, ExteriorError>;
}

pub struct ExteriorComputerInterface {
    computer: Computer,
}

impl ExteriorComputerInterface {
    pub fn new(computer: &Computer) -> ExteriorComputerInterface {
        ExteriorComputerInterface {
            computer: computer.clone(),
        }
    }
}

impl ExteriorInterface for ExteriorComputerInterface {
    fn get_picture(&mut self) -> Result<String, ExteriorError> {
        let mut result = String::from("");
        while let Some(pixel) = self.computer.next() {
            let code = pixel? as u8;
            let ch = code as char;
            result.push(ch)
        }

        Ok(result)
    }

    fn send_data(&mut self, data: &[String]) -> Result<i64, ExteriorError> {
        for line in data {
            for character in line.chars() {
                self.computer.provide_input(character as i64);
            }
            self.computer.provide_input(10);
        }
        self.computer.provide_input('n' as i64);
        self.computer.provide_input(10);

        self.computer.patch_memory(0, 2);
        let result = self.computer.run()?;

        if let Some(result) = result.last() {
            Ok(*result)
        } else {
            Err(ExteriorError::NoData)
        }
    }
}
