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
    Floor,
    Oxygen(usize),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Unknown => write!(f, " "),
            Tile::Wall => write!(f, "#"),
            Tile::Floor => write!(f, "."),
            Tile::Oxygen(_) => write!(f, "~"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Unexplored,
    Exploring,
    RepairedOxygenSystem,
    Oxygenizing,
    AllClear,
}

pub struct Droid<I> {
    _interface: I,
    _layout: HashMap<Pos, Tile>,
    _position: Pos,
    _status: Status,
}

impl<I> Droid<I> {
    fn get_tile(&self, pos: Pos) -> Tile {
        *self._layout.get(&pos).unwrap_or(&Tile::Unknown)
    }

    fn get_earliest_oxygen(&self, position: Pos, mn: Option<usize>) -> Option<usize> {
        if let Tile::Oxygen(level) = self.get_tile(position) {
            mn.map(|curr_min| curr_min.min(level)).or(Some(level))
        } else {
            mn
        }
    }

    fn get_oxygen_time(&self, position: Pos) -> Option<usize> {
        if let Tile::Oxygen(time) = self.get_tile(position) {
            return Some(time);
        }

        let mut earliest = None;

        earliest = self.get_earliest_oxygen(Direction::East + position, earliest);
        earliest = self.get_earliest_oxygen(Direction::North + position, earliest);
        earliest = self.get_earliest_oxygen(Direction::West + position, earliest);
        earliest = self.get_earliest_oxygen(Direction::South + position, earliest);

        earliest.map(|time| time + 1)
    }

    fn next_for_exploring(&self) -> Option<Direction> {
        let mut face_next = Direction::East;
        for _ in 0..4 {
            if self.get_tile(face_next + self._position) == Tile::Unknown {
                return Some(face_next);
            }
            face_next = face_next.turn_left();
        }
        None
    }

    fn next_for_oxygenizing(&self) -> Option<Direction> {
        let mut face_next = Direction::East;
        for _ in 0..4 {
            match self.get_tile(face_next + self._position) {
                Tile::Unknown | Tile::Floor => return Some(face_next),
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
            _interface: interface,
            _layout: HashMap::new(),
            _position: Pos::origin(),
            _status: Status::Unexplored,
        }
    }

    fn backtrack_exploring(
        &mut self,
        path: &mut Vec<Direction>,
    ) -> Result<Direction, ComputerError> {
        loop {
            if let Some(prev_dir) = path.pop() {
                let facing = prev_dir.turn_back();
                match self._interface.send_direction(facing)? {
                    Report::Moved => {
                        self._position = facing + self._position;
                        if let Some(facing) = self.next_for_exploring() {
                            return Ok(facing);
                        }
                    }

                    into => {
                        return Err(ComputerError::MessageError(format!(
                            "Droid error while backtracking into {:?}",
                            into
                        )));
                    }
                }
            } else {
                return Err(ComputerError::MessageError(format!("Backtrack to start")));
            }
        }
    }

    pub fn explore(&mut self) -> Result<usize, ComputerError> {
        if self._status != Status::Unexplored {
            return Err(ComputerError::MessageError(String::from(
                "Can only search in newly created szenario",
            )));
        }
        self._status = Status::Exploring;

        self._layout.insert(self._position, Tile::Floor);
        let mut path = Vec::new();
        let mut facing = self.next_for_exploring().unwrap_or(Direction::East);

        loop {
            match self._interface.send_direction(facing)? {
                Report::Wall => {
                    self._layout.insert(facing + self._position, Tile::Wall);
                    match self.next_for_exploring() {
                        Some(next_face) => facing = next_face,
                        None => facing = self.backtrack_exploring(&mut path)?,
                    }
                }

                Report::Moved => {
                    path.push(facing);
                    self._position = facing + self._position;
                    self._layout.insert(self._position, Tile::Floor);
                }

                Report::Oxygen => {
                    self._position = facing + self._position;
                    self._layout.insert(self._position, Tile::Oxygen(0));
                    self._status = Status::RepairedOxygenSystem;
                    return Ok(path.len() + 1);
                }
            }
        }
    }

    fn backtrack_oxygenizing(
        &mut self,
        path: &mut Vec<Direction>,
    ) -> Result<Option<Direction>, ComputerError> {
        loop {
            if let Some(prev_dir) = path.pop() {
                let facing = prev_dir.turn_back();
                match self._interface.send_direction(facing)? {
                    Report::Moved | Report::Oxygen => {
                        self._position = facing + self._position;
                        if let Some(facing) = self.next_for_oxygenizing() {
                            return Ok(Some(facing));
                        }
                    }

                    Report::Wall => {
                        return Err(ComputerError::MessageError(format!(
                            "Droid error while backtracking into Wall",
                        )))
                    }
                }
            } else {
                return Ok(None);
            }
        }
    }

    pub fn oxygenize(&mut self) -> Result<usize, ComputerError> {
        if self._status != Status::RepairedOxygenSystem {
            return Err(ComputerError::MessageError(String::from(
                "Can only oxygenize directly after I repaired the oxygen system",
            )));
        }
        self._status = Status::Oxygenizing;

        let mut facing = self.next_for_oxygenizing().unwrap_or(Direction::East);
        let mut path = Vec::new();
        let mut max_time: usize = 0;

        loop {
            match self._interface.send_direction(facing)? {
                Report::Wall => {
                    self._layout.insert(facing + self._position, Tile::Wall);
                    match self.next_for_oxygenizing() {
                        Some(next_face) => facing = next_face,
                        None => {
                            if let Some(next_face) = self.backtrack_oxygenizing(&mut path)? {
                                facing = next_face
                            } else {
                                self._status = Status::AllClear;
                                return Ok(max_time);
                            }
                        }
                    }
                }

                Report::Moved | Report::Oxygen => {
                    path.push(facing);
                    self._position = facing + self._position;
                    if let Some(time) = self.get_oxygen_time(self._position) {
                        max_time = max_time.max(time);
                        self._layout.insert(self._position, Tile::Oxygen(time));
                    } else {
                        return Err(ComputerError::MessageError(String::from(
                            "This tile is not oxygenized",
                        )));
                    }
                }
            }
        }
    }
}

impl<I> Display for Droid<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let area = self._layout.keys().copied().collect::<Area>();
        for row in area.rows(false) {
            for cell in row.cols(true) {
                if cell == self._position {
                    write!(f, "D")?;
                } else {
                    let tile = self.get_tile(cell);
                    write!(f, "{}", tile)?;
                }
            }
            writeln!(f, "")?;
        }
        writeln!(f, "--")
    }
}
