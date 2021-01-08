#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    fmt::Display,
};

use common::{Area as RawArea, Pos as RawPos};
use computer::{computer_error::ComputerError, Computer};

type Pos = RawPos<i64>;
type Area = RawArea<i64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<i64> for Tile {
    type Error = ComputerError;

    fn try_from(tile: i64) -> Result<Self, Self::Error> {
        match tile {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            _ => Err(ComputerError::MessageError(format!(
                "Unknown tile for game: {}",
                tile
            ))),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, " "),
            Tile::Wall => write!(f, "█"),
            Tile::Block => write!(f, "+"),
            Tile::Paddle => write!(f, "-"),
            Tile::Ball => write!(f, "o"),
        }
    }
}

pub enum Input {
    Tile(Pos, Tile),
    Score(i64),
}

pub struct Game {
    _board: HashMap<Pos, Tile>,
}

impl Game {
    fn get_tile(computer: &mut Computer) -> Result<Option<Input>, ComputerError> {
        let result = if let Some(result) = computer.do_steps(3)? {
            let x = result[0];
            let y = result[1];

            if x == -1 && y == 0 {
                Some(Input::Score(result[2]))
            } else {
                Some(Input::Tile(Pos::new(x, y), Tile::try_from(result[2])?))
            }
        } else {
            None
        };

        Ok(result)
    }

    pub fn count_type(&self, tile: Tile) -> usize {
        self._board.values().filter(|other| **other == tile).count()
    }

    pub fn parse(template: &Computer) -> Result<Game, ComputerError> {
        let mut _computer = template.clone();
        let mut _board = HashMap::new();
        while let Some(Input::Tile(pos, tile)) = Game::get_tile(&mut _computer)? {
            _board.insert(pos, tile);
        }

        Ok(Game { _board })
    }

    pub fn free_game(template: &Computer) -> Result<i64, ComputerError> {
        let mut computer = template.clone();
        computer.patch_memory(0, 2);

        loop {
            let mut score = 0i64;
            let mut blocks = HashSet::new();
            let mut paddle: Option<i64> = None;
            while let Some(input) = Game::get_tile(&mut computer)? {
                match input {
                    Input::Score(_score) => {
                        score = _score;
                    }
                    Input::Tile(pos, Tile::Block) => {
                        blocks.insert(pos);
                    }
                    Input::Tile(pos, Tile::Empty) => {
                        blocks.remove(&pos);
                    }
                    Input::Tile(pos, Tile::Paddle) => {
                        paddle = Some(pos.x());
                    }
                    Input::Tile(pos, Tile::Ball) => {
                        let direction = paddle
                            .map(|paddle| (pos.x() - paddle).signum())
                            .unwrap_or(0);
                        computer.provide_input(direction);
                    }
                    Input::Tile(_, Tile::Wall) => (),
                }
            }
            if blocks.is_empty() {
                return Ok(score);
            }
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let area = self._board.keys().copied().collect::<Area>();
        for row in area.rows() {
            for col in row.cols() {
                let tile = *self._board.get(&col).unwrap_or(&Tile::Empty);
                write!(f, "{}", tile)?;
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}
