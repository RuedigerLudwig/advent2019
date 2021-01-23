use std::error::Error;

use computer::{Code, TextInput, TextOutput, VirtualMachine};

use crate::exterior_error::ExteriorError;

pub trait ExteriorInterface {
    fn get_picture(&mut self) -> Result<Vec<String>, Box<dyn Error>>;
    fn send_data(&mut self, data: &[String], run_silent: bool) -> Result<i64, Box<dyn Error>>;
}

pub struct ExteriorComputerInterface<'a> {
    vm: VirtualMachine<'a, TextInput>,
    input: TextInput,
    output: TextOutput,
}

impl<'a> ExteriorComputerInterface<'a> {
    pub fn new(code: &'a Code) -> ExteriorComputerInterface {
        let input = TextInput::new();
        let vm = VirtualMachine::with_input(code, input.clone());
        let output = TextOutput::new(vm.get_output());
        ExteriorComputerInterface { vm, input, output }
    }
}

impl ExteriorInterface for ExteriorComputerInterface<'_> {
    fn get_picture(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut result = Vec::new();
        while let Some(line) = self.output.read_line()? {
            result.push(line)
        }

        Ok(result)
    }

    fn send_data(&mut self, data: &[String], run_silent: bool) -> Result<i64, Box<dyn Error>> {
        if !run_silent {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
        self.vm.patch_memory(0, 2);
        for answer in data {
            if let Some(question) = self.output.read_line()? {
                self.input.write_input(answer)?;
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

        if let Some(result) = self.output.next() {
            Ok(result?)
        } else {
            Err(Box::new(ExteriorError::NoData))
        }
    }
}
