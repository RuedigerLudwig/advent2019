use std::str::FromStr;

use common::{Direction, Pos};

use crate::error::WireError;

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    _direction: Direction,
    _steps: i32,
    _start: Pos<i32>,
}

impl Section {
    pub fn new(dir: Direction, steps: i32) -> Section {
        Section {
            _start: Pos::default(),
            _direction: dir,
            _steps: steps,
        }
    }

    pub fn steps(&self) -> i32 {
        self._steps
    }

    pub fn end(&self) -> Pos<i32> {
        self._start + self._direction.as_pos() * self._steps
    }

    pub fn set_start(&self, start: Pos<i32>) -> Section {
        Section {
            _start: start,
            _direction: self._direction,
            _steps: self._steps,
        }
    }

    pub fn distance(&self, pos: Pos<i32>) -> i32 {
        (pos - self._start).abs()
    }

    pub fn intersection(&self, other: &Section) -> Option<Pos<i32>> {
        if !self._direction.is_perpendicular(&other._direction) {
            None
        } else {
            let (left_right, down_up) =
                if self._direction == Direction::East || self._direction == Direction::West {
                    (self, other)
                } else {
                    (other, self)
                };
            let (left, right) = if left_right._direction == Direction::East {
                (left_right._start, left_right.end())
            } else {
                (left_right.end(), left_right._start)
            };

            let (down, up) = if down_up._direction == Direction::North {
                (down_up._start, down_up.end())
            } else {
                (down_up.end(), down_up._start)
            };

            let x = down.x();
            let y = left.y();

            if left.x() < x && x < right.x() && down.y() < y && y < up.y() {
                Some(Pos::new(x, y))
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
            let number = input[1..].parse()?;
            match input
                .chars()
                .next()
                .expect("We Checked, that we have at least one char")
            {
                'L' => Ok(Section::new(Direction::West, number)),
                'R' => Ok(Section::new(Direction::East, number)),
                'U' => Ok(Section::new(Direction::North, number)),
                'D' => Ok(Section::new(Direction::South, number)),
                _ => Err(WireError::ParseError(String::from(input))),
            }
        }
    }
}
