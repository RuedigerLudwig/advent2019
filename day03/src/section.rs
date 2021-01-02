use std::str::FromStr;

use crate::{pos::Pos, wire_error::WireError};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn as_pos(&self) -> Pos {
        match *self {
            Direction::Right => Pos::new(0, 1),
            Direction::Left => Pos::new(0, -1),
            Direction::Up => Pos::new(1, 0),
            Direction::Down => Pos::new(-1, 0),
        }
    }

    pub fn is_perpendicular(&self, other: &Direction) -> bool {
        match *self {
            Direction::Right => *other != Direction::Right && *other != Direction::Left,
            Direction::Left => *other != Direction::Right && *other != Direction::Left,
            Direction::Up => *other != Direction::Up && *other != Direction::Down,
            Direction::Down => *other != Direction::Up && *other != Direction::Down,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    _direction: Direction,
    _steps: i32,
    _start: Pos,
}

impl Section {
    pub fn new(dir: Direction, steps: i32) -> Section {
        Section {
            _start: Pos::origin(),
            _direction: dir,
            _steps: steps,
        }
    }

    pub fn steps(&self) -> i32 {
        self._steps
    }

    pub fn end(&self) -> Pos {
        self._start + self._direction.as_pos() * self._steps
    }

    pub fn set_start(&self, start: Pos) -> Section {
        Section {
            _start: start,
            _direction: self._direction,
            _steps: self._steps,
        }
    }

    pub fn distance(&self, pos: Pos) -> i32 {
        (pos - self._start).abs()
    }

    pub fn intersection(&self, other: &Section) -> Option<Pos> {
        if !self._direction.is_perpendicular(&other._direction) {
            None
        } else {
            let (left_right, down_up) =
                if self._direction == Direction::Right || self._direction == Direction::Left {
                    (self, other)
                } else {
                    (other, self)
                };
            let (left, right) = if left_right._direction == Direction::Right {
                (left_right._start, left_right.end())
            } else {
                (left_right.end(), left_right._start)
            };

            let (down, up) = if down_up._direction == Direction::Up {
                (down_up._start, down_up.end())
            } else {
                (down_up.end(), down_up._start)
            };

            let col = down.col();
            let row = left.row();

            if left.col() < col && col < right.col() && down.row() < row && row < up.row() {
                Some(Pos::new(row, col))
            } else {
                None
            }
        }
    }
}

impl FromStr for Section {
    type Err = WireError;

    fn from_str(input: &str) -> Result<Section, Self::Err> {
        if input.len() <= 1 {
            Err(WireError::ParseError(String::from(input)))
        } else {
            let number = input[1..].parse::<i32>()?;
            match input.chars().nth(0).unwrap() {
                'L' => Ok(Section::new(Direction::Left, number)),
                'R' => Ok(Section::new(Direction::Right, number)),
                'U' => Ok(Section::new(Direction::Up, number)),
                'D' => Ok(Section::new(Direction::Down, number)),
                _ => Err(WireError::ParseError(String::from(input))),
            }
        }
    }
}
