use crate::error::ExteriorError;
use computer::{Code, InputConverter, ListInput, TextVM};

pub trait ExteriorInterface {
    fn get_picture(&mut self) -> Result<Vec<String>, ExteriorError>;
    fn send_data(&mut self, data: &[String], run_silent: bool) -> Result<i64, ExteriorError>;
}

pub struct ExteriorComputerInterface<'a> {
    input: ListInput,
    vm: TextVM<'a>,
}

impl<'a> ExteriorComputerInterface<'a> {
    pub fn new(code: Code) -> ExteriorComputerInterface<'a> {
        let input = ListInput::new();
        let vm = TextVM::new(code, input.clone());
        ExteriorComputerInterface { input, vm }
    }
}

impl ExteriorInterface for ExteriorComputerInterface<'_> {
    fn get_picture(&mut self) -> Result<Vec<String>, ExteriorError> {
        let mut result = Vec::new();
        while let Some(line) = self.vm.read_line()? {
            result.push(line)
        }

        Ok(result)
    }

    fn send_data(&mut self, data: &[String], run_silent: bool) -> Result<i64, ExteriorError> {
        if !run_silent {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
        self.vm.patch_memory(0, 2);
        for answer in data {
            if let Some(question) = self.vm.read_line()? {
                answer.send_to(&mut self.input)?;
                if !run_silent {
                    println!("{} {}", question, answer);
                }
            }
        }

        let mut jump_start = true;
        while let Some(line) = self.vm.read_line()? {
            if !run_silent {
                if jump_start {
                    print!("{esc}[1;1H", esc = 27 as char);
                }
                println!("{}", line);
                jump_start = line.is_empty();
            }
        }

        if let Some(result) = self.vm.next()? {
            Ok(result)
        } else {
            Err(ExteriorError::NoData)
        }
    }
}
