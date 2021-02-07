use std::str::FromStr;

use common::{direction::Direction, pos::Pos};

use crate::error::WireError;

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    direction: Direction,
    steps: i32,
    start: Pos<i32>,
}

impl Section {
    pub fn new(direction: Direction, steps: i32) -> Section {
        Section {
            start: Pos::default(),
            direction,
            steps,
        }
    }

    pub fn steps(&self) -> i32 {
        self.steps
    }

    pub fn end(&self) -> Pos<i32> {
        self.start + self.direction.as_pos() * self.steps
    }

    pub fn set_start(&self, start: Pos<i32>) -> Section {
        Section {
            start,
            direction: self.direction,
            steps: self.steps,
        }
    }

    pub fn distance(&self, pos: Pos<i32>) -> i32 {
        (pos - self.start).abs()
    }

    pub fn intersection(&self, other: &Section) -> Option<Pos<i32>> {
        if !self.direction.is_perpendicular(&other.direction) {
            None
        } else {
            let (left_right, down_up) =
                if self.direction == Direction::East || self.direction == Direction::West {
                    (self, other)
                } else {
                    (other, self)
                };
            let (left, right) = if left_right.direction == Direction::East {
                (left_right.start, left_right.end())
            } else {
                (left_right.end(), left_right.start)
            };

            let (down, up) = if down_up.direction == Direction::North {
                (down_up.start, down_up.end())
            } else {
                (down_up.end(), down_up.start)
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
