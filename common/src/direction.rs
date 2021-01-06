use crate::Pos;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    East,
    West,
    North,
    South,
}

impl Direction {
    pub fn as_pos(&self) -> Pos<i32> {
        match *self {
            Direction::East => Pos::new(1, 0),
            Direction::West => Pos::new(-1, 0),
            Direction::North => Pos::new(0, 1),
            Direction::South => Pos::new(0, -1),
        }
    }

    pub fn is_perpendicular(&self, other: &Direction) -> bool {
        match *self {
            Direction::East => *other != Direction::East && *other != Direction::West,
            Direction::West => *other != Direction::East && *other != Direction::West,
            Direction::North => *other != Direction::North && *other != Direction::South,
            Direction::South => *other != Direction::North && *other != Direction::South,
        }
    }

    pub fn turn(&self, right: bool) -> Direction {
        match (*self, right) {
            (Direction::East, false) | (Direction::West, true) => Direction::North,
            (Direction::North, false) | (Direction::South, true) => Direction::West,
            (Direction::West, false) | (Direction::East, true) => Direction::South,
            (Direction::South, false) | (Direction::North, true) => Direction::East,
        }
    }
}
