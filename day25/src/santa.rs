use computer::{Code, ComputerError, InputConverter, ListInput, TextVM};

#[derive(Debug)]
pub struct SantasShip<'a> {
    input: ListInput,
    vm: TextVM<'a>,
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
        let vm = TextVM::new(code, input.clone());

        SantasShip { input, vm }
    }

    pub fn get_text(&mut self) -> Result<(ShipState, Vec<String>), ComputerError> {
        let mut lines = Vec::new();
        let mut last_line = "".to_owned();
        while let Some(line) = self.vm.read_line()? {
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
        command.trim().send_to(&mut self.input)
    }
}
