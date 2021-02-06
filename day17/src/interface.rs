use computer::{Code, InputConverter, ListInput, STTextOutput, STVirtualMachine};

use crate::error::ExteriorError;

pub trait ExteriorInterface {
    fn get_picture(&self) -> Result<Vec<String>, ExteriorError>;
    fn send_data(&self, data: &[String], run_silent: bool) -> Result<i64, ExteriorError>;
}

pub struct ExteriorComputerInterface<'a> {
    vm: STVirtualMachine<'a>,
    output: STTextOutput<'a>,
}

impl<'a> ExteriorComputerInterface<'a> {
    pub fn new(code: Code) -> ExteriorComputerInterface<'a> {
        let input = ListInput::new();
        let vm = STVirtualMachine::new(code, input);
        let output = STTextOutput::new(vm.get_output());
        ExteriorComputerInterface { vm, output }
    }
}

impl ExteriorInterface for ExteriorComputerInterface<'_> {
    fn get_picture(&self) -> Result<Vec<String>, ExteriorError> {
        let mut result = Vec::new();
        while let Some(line) = self.output.read_line()? {
            result.push(line)
        }

        Ok(result)
    }

    fn send_data(&self, data: &[String], run_silent: bool) -> Result<i64, ExteriorError> {
        if !run_silent {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
        self.vm.patch_memory(0, 2);
        for answer in data {
            if let Some(question) = self.output.read_line()? {
                answer.send_to(&self.vm)?;
                if !run_silent {
                    println!("{} {}", question, answer);
                }
            }
        }

        let mut jump_start = true;
        while let Some(line) = self.output.read_line()? {
            if !run_silent {
                if jump_start {
                    print!("{esc}[1;1H", esc = 27 as char);
                }
                println!("{}", line);
                jump_start = line.is_empty();
            }
        }

        if let Some(result) = self.output.int_value()? {
            Ok(result)
        } else {
            Err(ExteriorError::NoData)
        }
    }
}
