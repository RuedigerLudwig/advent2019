use crate::{Pos, Turn};
use std::{
    fmt::Display,
    ops::{Add, Sub},
};
use Turn::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    East,
    North,
    West,
    South,
}

use Direction::*;

impl Direction {
    pub fn as_pos(&self) -> Pos<i32> {
        match *self {
            East => Pos::new(1, 0),
            North => Pos::new(0, 1),
            West => Pos::new(-1, 0),
            South => Pos::new(0, -1),
        }
    }

    pub fn is_perpendicular(&self, other: &Direction) -> bool {
        match *self {
            East => *other != East && *other != West,
            North => *other != North && *other != South,
            West => *other != East && *other != West,
            South => *other != North && *other != South,
        }
    }

    pub fn get_turn(&self, toward: Direction) -> Turn {
        if *self == toward {
            Forward
        } else if toward == self.turn_left() {
            Left
        } else if toward == self.turn_right() {
            Right
        } else {
            Back
        }
    }

    pub fn turn(&self, turn: Turn) -> Direction {
        match turn {
            Left => self.turn_left(),
            Right => self.turn_right(),
            Back => self.turn_back(),
            Forward => *self,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match *self {
            East => South,
            North => East,
            West => North,
            South => West,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match *self {
            East => North,
            North => West,
            West => South,
            South => East,
        }
    }

    pub fn turn_back(&self) -> Direction {
        match *self {
            East => West,
            North => South,
            West => East,
            South => North,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Direction::East => write!(f, "East"),
            Direction::North => write!(f, "North"),
            Direction::West => write!(f, "West"),
            Direction::South => write!(f, "South"),
        }
    }
}

impl Add<Pos<i32>> for Direction {
    type Output = Pos<i32>;

    fn add(self, rhs: Pos<i32>) -> Self::Output {
        rhs + self.as_pos()
    }
}

impl Add<Pos<i32>> for &Direction {
    type Output = Pos<i32>;

    fn add(self, rhs: Pos<i32>) -> Self::Output {
        rhs + self.as_pos()
    }
}

impl Add<Turn> for Direction {
    type Output = Self;

    fn add(self, rhs: Turn) -> Self {
        self.turn(rhs)
    }
}

impl Add<&Pos<i32>> for Direction {
    type Output = Pos<i32>;

    fn add(self, rhs: &Pos<i32>) -> Self::Output {
        *rhs + self.as_pos()
    }
}

impl Sub<Direction> for Direction {
    type Output = Turn;

    fn sub(self, rhs: Direction) -> Self::Output {
        self.get_turn(rhs)
    }
}
