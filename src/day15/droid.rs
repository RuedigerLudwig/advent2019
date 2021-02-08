use crate::common::pos::Pos as RawPos;
use crate::common::{area::Area as RawArea, direction::Direction};
use std::{collections::HashMap, fmt::Display};

use super::{
    error::DroidError,
    interface::{DroidComputerInterface, Report},
};

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

#[derive(Debug)]
pub struct Droid<I> {
    interface: I,
    layout: HashMap<Pos, Tile>,
    position: Pos,
    status: Status,
}

impl<I> Droid<I> {
    fn get_tile(&self, pos: Pos) -> Tile {
        *self.layout.get(&pos).unwrap_or(&Tile::Unknown)
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
            if self.get_tile(self.position + face_next) == Tile::Unknown {
                return Some(face_next);
            }
            face_next = face_next.turn_left();
        }
        None
    }

    fn next_for_oxygenizing(&self) -> Option<Direction> {
        let mut face_next = Direction::East;
        for _ in 0..4 {
            match self.get_tile(self.position + face_next) {
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
            interface,
            layout: HashMap::new(),
            position: Pos::default(),
            status: Status::Unexplored,
        }
    }

    fn backtrack_exploring(&mut self, path: &mut Vec<Direction>) -> Result<Direction, DroidError> {
        loop {
            if let Some(prev_dir) = path.pop() {
                let facing = prev_dir.turn_back();
                match self.interface.send_direction(facing)? {
                    Report::Moved => {
                        self.position = self.position + facing;
                        if let Some(facing) = self.next_for_exploring() {
                            return Ok(facing);
                        }
                    }

                    into => {
                        return Err(DroidError::BacktracingInto(into));
                    }
                }
            } else {
                return Err(DroidError::BacktracingToStart);
            }
        }
    }

    pub fn explore(&mut self) -> Result<usize, DroidError> {
        if self.status != Status::Unexplored {
            return Err(DroidError::AlreadyExplored);
        }
        self.status = Status::Exploring;

        self.layout.insert(self.position, Tile::Floor);
        let mut path = Vec::new();
        let mut facing = self.next_for_exploring().unwrap_or(Direction::East);

        loop {
            match self.interface.send_direction(facing)? {
                Report::Wall => {
                    self.layout.insert(self.position + facing, Tile::Wall);
                    facing = match self.next_for_exploring() {
                        Some(next_face) => next_face,
                        None => self.backtrack_exploring(&mut path)?,
                    }
                }

                Report::Moved => {
                    path.push(facing);
                    self.position = self.position + facing;
                    self.layout.insert(self.position, Tile::Floor);
                }

                Report::Oxygen => {
                    self.position = self.position + facing;
                    self.layout.insert(self.position, Tile::Oxygen(0));
                    self.status = Status::RepairedOxygenSystem;
                    return Ok(path.len() + 1);
                }
            }
        }
    }

    fn backtrack_oxygenizing(
        &mut self,
        path: &mut Vec<Direction>,
    ) -> Result<Option<Direction>, DroidError> {
        while let Some(prev_dir) = path.pop() {
            let facing = prev_dir.turn_back();

            if matches!(
                self.interface.send_direction(facing)?,
                Report::Moved | Report::Oxygen
            ) {
                self.position = self.position + facing;
                if let Some(facing) = self.next_for_oxygenizing() {
                    return Ok(Some(facing));
                }
            } else {
                return Err(DroidError::BacktracingInto(Report::Wall));
            }
        }
        Ok(None)
    }

    pub fn oxygenize(&mut self) -> Result<usize, DroidError> {
        if self.status != Status::RepairedOxygenSystem {
            return Err(DroidError::NotReadyToOxygenize);
        }
        self.status = Status::Oxygenizing;

        let mut facing = self.next_for_oxygenizing().unwrap_or(Direction::East);
        let mut path = Vec::new();
        let mut max_time: usize = 0;

        loop {
            match self.interface.send_direction(facing)? {
                Report::Wall => {
                    self.layout.insert(self.position + facing, Tile::Wall);
                    match self.next_for_oxygenizing() {
                        Some(next_face) => facing = next_face,
                        None => {
                            if let Some(next_face) = self.backtrack_oxygenizing(&mut path)? {
                                facing = next_face
                            } else {
                                self.status = Status::AllClear;
                                return Ok(max_time);
                            }
                        }
                    }
                }

                Report::Moved | Report::Oxygen => {
                    path.push(facing);
                    self.position = self.position + facing;
                    if let Some(time) = self.get_oxygen_time(self.position) {
                        max_time = max_time.max(time);
                        self.layout.insert(self.position, Tile::Oxygen(time));
                    } else {
                        return Err(DroidError::NotOxygenized);
                    }
                }
            }
        }
    }
}

impl<I> Display for Droid<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let area = self.layout.keys().copied().collect::<Area>();
        for row in area.rows(false) {
            for cell in row.cols(true) {
                if cell == self.position {
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
