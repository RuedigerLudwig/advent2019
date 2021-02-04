use crate::{
    error::DroneError,
    santa::{SantasShip, ShipState},
};
use common::Direction;
use computer::Code;

#[derive(Debug)]
struct Room {
    name: String,
    exits: Vec<Direction>,
    items: Vec<String>,
}

#[derive(Debug)]
pub struct Drone<'a> {
    _code: &'a Code,
    _ship: SantasShip<'a>,
    _carrying: Vec<String>,
    _avoid: Vec<String>,
}

#[derive(Debug)]
enum ExploreResult {
    Restart,
    SecurityPath(Vec<Direction>),
    DeadEnd,
}

enum SecurityCheck {
    Pass(String),
    TooHeavy,
    TooLight,
}

impl<'a> Drone<'a> {
    pub fn new(code: &'a Code) -> Drone<'a> {
        Drone {
            _code: code,
            _ship: SantasShip::new(code),
            _carrying: Vec::new(),
            _avoid: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<String, DroneError> {
        let path = loop {
            match self.explore(None)? {
                ExploreResult::Restart => self.restart(),
                ExploreResult::DeadEnd => return Err(DroneError::NoSecurityCheckpoint),
                ExploreResult::SecurityPath(path) => break path,
            }
        };

        let direction = self.go_to_security_check(&path)?;
        let password = self.try_to_pass(direction)?;

        Ok(password)
    }

    fn explore(&mut self, from: Option<Direction>) -> Result<ExploreResult, DroneError> {
        let (_, lines) = self._ship.get_text()?;

        if let Some(room) = self.analyse_room(&lines) {
            let mut result = ExploreResult::DeadEnd;

            for item in &room.items {
                if !self._avoid.contains(item) {
                    if !self.take_item_and_check(item)? {
                        return Ok(ExploreResult::Restart);
                    }
                }
            }

            if room.name != "Security Checkpoint" {
                for exit in &room.exits {
                    if from.map(|from| from != *exit).unwrap_or(true) {
                        self.say_direction(*exit)?;
                        let dir_result = self.explore(Some(exit.turn_back()))?;
                        match dir_result {
                            ExploreResult::Restart => return Ok(ExploreResult::Restart),
                            ExploreResult::SecurityPath(_) => result = dir_result,
                            ExploreResult::DeadEnd => {}
                        }
                    }
                }
            } else {
                result = ExploreResult::SecurityPath(vec![]);
            }

            if let Some(from) = from {
                if let ExploreResult::SecurityPath(mut path) = result {
                    path.push(from.turn_back());
                    result = ExploreResult::SecurityPath(path);
                }

                self.say_direction(from)?;
                self._ship.get_text()?;
            }
            Ok(result)
        } else {
            Err(DroneError::InvalidRoomDescription)
        }
    }

    fn restart(&mut self) {
        self._ship = SantasShip::new(self._code);
        self._carrying = Vec::new();
    }

    fn analyse_room(&self, lines: &[String]) -> Option<Room> {
        let mut exits = Vec::new();
        let mut items = Vec::new();
        let mut reading_doors = false;
        let mut reading_items = false;
        let mut name = None;
        for line in lines {
            if line.is_empty() {
                continue;
            } else if line.starts_with("== ") {
                name = Some(line[3..line.len() - 3].to_owned());
            } else if line == "Doors here lead:" {
                reading_doors = true;
                reading_items = false;
            } else if line == "Items here:" {
                reading_doors = false;
                reading_items = true;
            } else if line.starts_with("- ") {
                if reading_doors {
                    let text = &line[2..];
                    if text == "east" {
                        exits.push(Direction::East);
                    } else if text == "north" {
                        exits.push(Direction::North);
                    } else if text == "west" {
                        exits.push(Direction::West);
                    } else if text == "south" {
                        exits.push(Direction::South);
                    }
                } else if reading_items {
                    let text = line[2..].to_owned();
                    items.push(text);
                }
            }
        }

        name.map(|name| Room { name, exits, items })
    }

    fn take_item_and_check(&mut self, item: &String) -> Result<bool, DroneError> {
        self._ship.send_command(&format!("take {}", item))?;
        let (state, _) = self._ship.get_text()?;

        if let ShipState::Text = state {
            self._ship.send_command("inv")?;
            let (_, lines) = self._ship.get_text()?;

            for line in lines {
                if line.starts_with("- ") && &line[2..] == item {
                    self._carrying.push(item.to_owned());
                    return Ok(true);
                }
            }
        }

        self._avoid.push(item.to_owned());
        Ok(false)
    }

    fn go_to_security_check(&mut self, path: &[Direction]) -> Result<Direction, DroneError> {
        let mut last_lines = None;

        for dir in path.iter().rev() {
            self.say_direction(*dir)?;
            let (_, lines) = self._ship.get_text()?;
            last_lines = Some(lines);
        }

        let last_lines = last_lines.unwrap();
        let room = self.analyse_room(&last_lines).unwrap();
        for exit in room.exits {
            if exit != path[0].turn_back() {
                return Ok(exit);
            }
        }

        Err(DroneError::NoSecurityExit)
    }

    fn drop_single_item(&mut self, item: &str) -> Result<(), DroneError> {
        self._ship.send_command(&format!("drop {}", item))?;
        self._ship.get_text()?;
        self._carrying.retain(|carry| carry != item);

        Ok(())
    }

    fn drop_all(&mut self) -> Result<(), DroneError> {
        for item in &self._carrying {
            self._ship.send_command(&format!("drop {}", item))?;
            self._ship.get_text()?;
        }
        self._carrying.clear();
        Ok(())
    }

    fn take_item(&mut self, item: &str) -> Result<(), DroneError> {
        self._ship.send_command(&format!("take {}", item))?;
        self._ship.get_text()?;
        self._carrying.push(item.to_owned());
        Ok(())
    }

    fn analyze_security_output(&self, lines: &[String]) -> Result<SecurityCheck, DroneError> {
        for line in lines {
            if line.contains("lighter") {
                return Ok(SecurityCheck::TooHeavy);
            } else if line.contains("heavier") {
                return Ok(SecurityCheck::TooLight);
            } else if line.starts_with("\"Oh, hello!") {
                let password = line
                    .chars()
                    .filter(|ch| ch.is_digit(10))
                    .collect::<String>();

                return Ok(SecurityCheck::Pass(password));
            }
        }
        return Err(DroneError::UnknownSecurityMessage);
    }

    fn test_with_weights(
        &mut self,
        items: &[String],
        direction: Direction,
    ) -> Result<Option<String>, DroneError> {
        if items.is_empty() {
            return Ok(None);
        }

        let item = &items[0];
        self.take_item(item)?;

        self.say_direction(direction)?;
        let (_, lines) = self._ship.get_text()?;

        match self.analyze_security_output(&lines)? {
            SecurityCheck::TooHeavy => (),

            SecurityCheck::TooLight => {
                if let Some(result) = self.test_with_weights(&items[1..], direction)? {
                    return Ok(Some(result));
                }
            }

            SecurityCheck::Pass(result) => return Ok(Some(result)),
        }

        self.drop_single_item(item)?;
        self.test_with_weights(&items[1..], direction)
    }

    fn try_to_pass(&mut self, direction: Direction) -> Result<String, DroneError> {
        let choices = self._carrying.clone();
        self.drop_all()?;

        if let Some(result) = self.test_with_weights(&choices, direction)? {
            return Ok(result);
        }

        Err(DroneError::NoWeightFits)
    }

    fn say_direction(&self, dir: Direction) -> Result<(), DroneError> {
        let direction = match dir {
            Direction::East => "east",
            Direction::North => "north",
            Direction::West => "west",
            Direction::South => "south",
        };
        self._ship.send_command(direction)?;
        Ok(())
    }
}
