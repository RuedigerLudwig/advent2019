use computer::Computer;

pub trait BotComputerInterface {
    fn accept_input(&mut self, is_white: bool) -> Option<(bool, bool)>;
}

pub struct ComputerInterface {
    computer: Computer,
}

impl ComputerInterface {
    pub fn new(computer: &Computer) -> ComputerInterface {
        ComputerInterface {
            computer: computer.clone(),
        }
    }
}

impl BotComputerInterface for ComputerInterface {
    fn accept_input(&mut self, is_white: bool) -> Option<(bool, bool)> {
        self.computer.provide_input(if is_white { 1 } else { 0 });
        let paint = match self.computer.next() {
            Some(Ok(paint)) => paint == 1,
            _ => return None,
        };
        let turn = match self.computer.next() {
            Some(Ok(turn)) => turn == 1,
            _ => return None,
        };
        Some((paint, turn))
    }
}
