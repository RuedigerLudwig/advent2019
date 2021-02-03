use crate::{
    error::DroneError,
    santa::{SantasShip, ShipState},
};
use computer::Code;
use std::io::{self, prelude::*};

#[derive(Debug)]
pub struct Interactive {
    _ship: SantasShip,
}

impl Interactive {
    pub fn new(code: &Code) -> Interactive {
        Interactive {
            _ship: SantasShip::new(code),
        }
    }

    pub fn run(&mut self) -> Result<(), DroneError> {
        let stdin = io::stdin();
        loop {
            let (state, lines) = self._ship.get_text()?;
            for line in lines {
                println!("{}", line);
            }
            match state {
                ShipState::Text => {
                    let mut text = String::new();
                    stdin.lock().read_line(&mut text)?;
                    self._ship.send_command(&text)?;
                }
                ShipState::Crash => {
                    println!("Crash!");
                    return Ok(());
                }
                ShipState::Loop => {
                    println!("Loop!");
                    return Ok(());
                }
            }
        }
    }
}
