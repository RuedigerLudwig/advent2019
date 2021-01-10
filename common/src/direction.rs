use crate::Pos;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    East,
    North,
    West,
    South,
}

impl Direction {
    pub fn as_pos(&self) -> Pos<i32> {
        match *self {
            Direction::East => Pos::new(1, 0),
            Direction::North => Pos::new(0, 1),
            Direction::West => Pos::new(-1, 0),
            Direction::South => Pos::new(0, -1),
        }
    }

    pub fn is_perpendicular(&self, other: &Direction) -> bool {
        match *self {
            Direction::East => *other != Direction::East && *other != Direction::West,
            Direction::North => *other != Direction::North && *other != Direction::South,
            Direction::West => *other != Direction::East && *other != Direction::West,
            Direction::South => *other != Direction::North && *other != Direction::South,
        }
    }

    pub fn turn(&self, to_right: bool) -> Direction {
        match (*self, to_right) {
            (Direction::East, false) | (Direction::West, true) => Direction::North,
            (Direction::North, false) | (Direction::South, true) => Direction::West,
            (Direction::West, false) | (Direction::East, true) => Direction::South,
            (Direction::South, false) | (Direction::North, true) => Direction::East,
        }
    }

    pub fn turn_right(&self) -> Direction {
        self.turn(true)
    }

    pub fn turn_left(&self) -> Direction {
        self.turn(false)
    }

    pub fn turn_back(&self) -> Direction {
        match *self {
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::North => Direction::South,
        }
    }
}

impl Add<Pos<i32>> for Direction {
    type Output = Pos<i32>;

    fn add(self, rhs: Pos<i32>) -> Self::Output {
        rhs + self.as_pos()
    }
}
