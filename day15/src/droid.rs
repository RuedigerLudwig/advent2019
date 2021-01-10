#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display};

use common::Pos as RawPos;
use common::{Area as RawArea, Direction};
use computer::computer_error::ComputerError;

use crate::interface::{DroidComputerInterface, Report};

type Pos = RawPos<i32>;
type Area = RawArea<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Unknown,
    Wall,
    UnfinishedFloor,
    DeadEnd,
    Oxygen,
    Start,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Unknown => write!(f, " "),
            Tile::Wall => write!(f, "#"),
            Tile::UnfinishedFloor => write!(f, "."),
            Tile::DeadEnd => write!(f, "."),
            Tile::Oxygen => write!(f, "O"),
            Tile::Start => write!(f, "S"),
        }
    }
}

pub struct Droid<I> {
    interface: I,
    layout: HashMap<Pos, Tile>,
    position: Pos,
    path: Vec<Direction>,
}

impl<I> Droid<I> {
    fn get_tile(&self, pos: Pos) -> Tile {
        *self.layout.get(&pos).unwrap_or(&Tile::Unknown)
    }

    fn get_tile_info(&self, position: Pos) -> HashMap<Direction, Tile> {
        let mut result = HashMap::new();
        result.insert(Direction::East, self.get_tile(Direction::East + position));
        result.insert(Direction::North, self.get_tile(Direction::North + position));
        result.insert(Direction::West, self.get_tile(Direction::West + position));
        result.insert(Direction::South, self.get_tile(Direction::South + position));
        result
    }

    fn do_turn(&self, facing: Direction) -> Option<Direction> {
        let mut face_next = facing;
        for _ in 0..4 {
            match self.get_tile(face_next + self.position) {
                Tile::Unknown => return Some(face_next),
                _ => (),
            }
            face_next = face_next.turn_left();
        }
        None
    }
}

impl<I> Droid<I>
where
    I: DroidComputerInterface,
{
    pub fn new(interface: I) -> Droid<I> {
        Droid {
            interface,
            layout: HashMap::new(),
            position: Pos::origin(),
            path: Vec::new(),
        }
    }

    pub fn backtrack(&mut self) -> Result<Direction, ComputerError> {
        loop {
            self.layout.insert(self.position, Tile::DeadEnd);
            if let Some(prev_dir) = self.path.pop() {
                let facing = prev_dir.turn_back();
                match self.interface.send_direction(facing)? {
                    Report::Moved => {
                        self.position = facing + self.position;
                        if let Some(facing) = self.do_turn(facing) {
                            return Ok(facing);
                        }
                    }
                    err => {
                        return Err(ComputerError::MessageError(format!(
                            "Droid error while backtracking: {:?}",
                            err
                        )));
                    }
                }
            } else {
                return Err(ComputerError::MessageError(format!("Backtrack to start")));
            }
        }
    }

    pub fn explore(&mut self) -> Result<usize, ComputerError> {
        let mut facing = Direction::East;
        self.layout.insert(self.position, Tile::Start);
        let result = loop {
            match self.interface.send_direction(facing)? {
                Report::Wall => {
                    self.layout.insert(facing + self.position, Tile::Wall);
                    match self.do_turn(facing) {
                        Some(next_face) => facing = next_face,
                        None => facing = self.backtrack()?,
                    }
                }
                Report::Moved => {
                    self.path.push(facing);
                    self.position = facing + self.position;
                    self.layout
                        .entry(self.position)
                        .or_insert(Tile::UnfinishedFloor);
                }
                Report::Oxygen => {
                    self.layout.insert(facing + self.position, Tile::Oxygen);
                    break self.path.len() + 1;
                }
            }
        };

        Ok(result)
    }
}

impl<I> Display for Droid<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let area = self.layout.keys().copied().collect::<Area>();
        for row in area.rows(false) {
            for col in row.cols(true) {
                let tile = self.get_tile(col);
                if col == self.position {
                    if tile == Tile::Oxygen {
                        write!(f, "H")?;
                    } else {
                        write!(f, "D")?;
                    }
                } else {
                    write!(f, "{}", tile)?;
                }
            }
            writeln!(f, "")?;
        }
        writeln!(f, "--")
    }
}

#[cfg(test)]
mod tests {
    use computer::Computer;

    use crate::interface::ComputerInterface;

    use super::*;

    #[test]
    fn parse_simple() -> Result<(), ComputerError> {
        let template = Computer::from_file("day15", "input.txt")?;
        let interface = ComputerInterface::new(&template);
        let mut droid = Droid::new(interface);
        droid.explore()?;

        Ok(())
    }
}
