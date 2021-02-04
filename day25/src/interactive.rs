use crate::{
    error::DroneError,
    santa::{SantasShip, ShipState},
};
use common::join;
use computer::Code;
use std::io::{self, prelude::*};

const COMMANDS: [&'static str; 11] = [
    "east", "north", "west", "south", "take", "drop", "inv", "last", "quit", "restart", "help",
];

const DESCRIPTIONS: [&'static str; 11] = [
    "moves to the east",
    "moves to the north",
    "moves to the west",
    "moves to the south",
    "takes an item",
    "drops an item",
    "shows the inventory",
    "shows the last room description",
    "exits the game",
    "restarts the game",
    "shows this help",
];

#[derive(Debug)]
pub struct Interactive<'a> {
    _code: &'a Code,
    _ship: SantasShip<'a>,
}

impl<'a> Interactive<'a> {
    pub fn new(code: &'a Code) -> Interactive<'a> {
        Interactive {
            _code: code,
            _ship: SantasShip::new(code),
        }
    }

    pub fn run(&mut self) -> Result<(), DroneError> {
        let mut room_desc = None;
        loop {
            let (state, lines) = self._ship.get_text()?;
            let mut is_room_desc = false;
            for line in &lines {
                println!("{}", line);
                if line.starts_with("==") {
                    is_room_desc = true;
                }
            }
            if is_room_desc {
                room_desc = Some(lines);
            }
            match state {
                ShipState::Text => {
                    let text = loop {
                        let text = self.get_input()?;
                        match text.as_str() {
                            "?" => {
                                println!("Possible Commands:");
                                println!("{}", join(&COMMANDS.to_vec(), ", "));
                                println!("\nCommand?");
                            }
                            "help" => {
                                println!("Possible Commands:");
                                for i in 0..COMMANDS.len() {
                                    println!("  {:7} - {}", COMMANDS[i], DESCRIPTIONS[i]);
                                }
                                println!("All commands can be abbriviated");
                                println!("\nCommand?");
                            }
                            "last" => {
                                if let Some(ref lines) = room_desc {
                                    for line in lines {
                                        println!("{}", line);
                                    }
                                } else {
                                    println!("There is no curent room description");
                                    println!("\nCommand?");
                                }
                            }
                            "quit" => return Ok(()),
                            _ => break text,
                        }
                    };

                    if text == "restart" {
                        self._ship = SantasShip::new(self._code);
                    } else {
                        self._ship.send_command(&text)?;
                    }
                }

                ShipState::Crash | ShipState::Loop => {
                    if let ShipState::Crash = state {
                        println!("You crashed the game");
                    } else {
                        println!("The last message loops forever");
                    }
                    loop {
                        println!("\nType 'restart' or 'quit'");
                        let text = self.get_input()?;

                        match text.as_str() {
                            "quit" => return Ok(()),
                            "restart" => {
                                self._ship = SantasShip::new(self._code);
                                break;
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    fn get_input(&self) -> Result<String, DroneError> {
        let stdin = io::stdin();
        let mut text = String::new();
        stdin.lock().read_line(&mut text)?;

        let mut splits = text.trim().split(char::is_whitespace).collect::<Vec<_>>();
        if splits.len() > 0 {
            let first = splits[0];
            for i in 0..COMMANDS.len() {
                let command = COMMANDS[i];
                if command == first {
                    return Ok(text);
                } else if command.starts_with(&first) {
                    splits[0] = command;
                    return Ok(join(&splits, " "));
                }
            }
        }

        Ok(text.trim().to_owned())
    }
}
