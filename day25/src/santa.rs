use computer::{Code, ComputerError, InputConverter, ListInput, STTextVM};

#[derive(Debug)]
pub struct SantasShip<'a> {
    _input: ListInput,
    _vm: STTextVM<'a>,
}

#[derive(Debug)]
pub enum ShipState {
    Crash,
    Loop,
    Text,
}

impl<'a> SantasShip<'a> {
    pub fn new(code: Code) -> SantasShip<'a> {
        let input = ListInput::new();
        let vm = STTextVM::new_single(code, input.clone());

        SantasShip {
            _input: input,
            _vm: vm,
        }
    }

    pub fn get_text(&self) -> Result<(ShipState, Vec<String>), ComputerError> {
        let mut lines = Vec::new();
        let mut last_line = "".to_owned();
        while let Some(line) = self._vm.read_line()? {
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

    pub fn send_command(&mut self, command: &str) -> Result<(), ComputerError> {
        command.trim().send_to(&mut self._input)
    }
}
