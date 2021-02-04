use computer::{Code, ComputerError, InputConverter, ListInput, STTextOutput, STVirtualMachine};

#[derive(Debug)]
pub struct SantasShip<'a> {
    _output: STTextOutput<'a>,
    _vm: STVirtualMachine<'a>,
}

#[derive(Debug)]
pub enum ShipState {
    Crash,
    Loop,
    Text,
}

impl SantasShip<'_> {
    pub fn new(code: &Code) -> SantasShip<'_> {
        let input = ListInput::new();
        let vm = STVirtualMachine::new(&code, input);
        let output = STTextOutput::new(vm.get_output());

        SantasShip {
            _output: output,
            _vm: vm,
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
        command.trim().send_to(&self._vm)
    }
}
