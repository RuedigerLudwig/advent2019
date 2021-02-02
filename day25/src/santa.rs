#![allow(dead_code)]

use computer::{Code, ComputerError, TextInput, TextOutput, VirtualMachine};

#[derive(Debug)]
pub struct SantasShip {
    _input: TextInput,
    _output: TextOutput,
}

#[derive(Debug)]
pub enum ShipState {
    Crash,
    Loop,
    Text,
}

impl SantasShip {
    pub fn new(code: &Code) -> SantasShip {
        let input = TextInput::new();
        let vm = VirtualMachine::new(&code, &input);
        let output = TextOutput::new(vm.get_output());

        SantasShip {
            _input: input,
            _output: output,
        }
    }

    pub fn get_text(&self) -> Result<(ShipState, Vec<String>), ComputerError> {
        let mut lines = Vec::new();
        let mut last_line = "".to_owned();
        while let Some(line) = self._output.read_line()? {
            lines.push(line.to_owned());
            if !line.is_empty() {
                if line == last_line {
                    return Ok((ShipState::Loop, lines));
                }
                last_line = line.to_owned();
            }
            if line == "Command?" {
                return Ok((ShipState::Text, lines));
            }
        }

        Ok((ShipState::Crash, lines))
    }

    pub fn send_command(&self, command: &str) -> Result<(), ComputerError> {
        self._input.write_input(command.trim())?;
        Ok(())
    }
}
