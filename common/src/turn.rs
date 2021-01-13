use crate::Direction;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    Left,
    Right,
    Back,
    Forward,
}

use Turn::*;

impl Turn {
    pub fn to_left(&self) -> Turn {
        match *self {
            Left => Back,
            Back => Right,
            Right => Forward,
            Forward => Left,
        }
    }

    pub fn to_right(&self) -> Turn {
        match *self {
            Left => Forward,
            Back => Left,
            Right => Back,
            Forward => Right,
        }
    }

    pub fn to_back(&self) -> Turn {
        match *self {
            Left => Right,
            Back => Forward,
            Right => Left,
            Forward => Back,
        }
    }
}

impl Add<Turn> for Turn {
    type Output = Turn;

    fn add(self, rhs: Turn) -> Self::Output {
        match rhs {
            Left => self.to_left(),
            Back => self.to_back(),
            Right => self.to_right(),
            Forward => self,
        }
    }
}

impl Add<Direction> for Turn {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Self::Output {
        rhs.turn(self)
    }
}
