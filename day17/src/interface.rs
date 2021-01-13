use computer::Computer;

use crate::exterior_error::ExteriorError;

pub trait ExteriorInterface {
    fn get_picture(&mut self) -> Result<Vec<String>, ExteriorError>;
    fn send_data(&mut self, data: &[String], run_silent: bool) -> Result<i64, ExteriorError>;
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
    fn get_picture(&mut self) -> Result<Vec<String>, ExteriorError> {
        let mut result = Vec::new();
        while let Some(line) = self.computer.read_output()? {
            result.push(line)
        }

        Ok(result)
    }

    fn send_data(&mut self, data: &[String], run_silent: bool) -> Result<i64, ExteriorError> {
        if !run_silent {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
        self.computer.patch_memory(0, 2);
        for answer in data {
            if let Some(question) = self.computer.read_output()? {
                self.computer.write_input(answer)?;
                if !run_silent {
                    println!("{} {}", question, answer);
                }
            }
        }

        if !run_silent {
            let mut jump_start = true;
            while let Some(line) = self.computer.read_output()? {
                if jump_start {
                    print!("{esc}[1;1H", esc = 27 as char);
                }
                println!("{}", line);
                jump_start = line.is_empty();
            }
        }
        let result = self.computer.run()?;

        if let Some(result) = result.last() {
            Ok(*result)
        } else {
            Err(ExteriorError::NoData)
        }
    }
}
