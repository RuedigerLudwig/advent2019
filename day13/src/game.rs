#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    fmt::Display,
};

use common::{Area as RawArea, Pos as RawPos};
use computer::{Code, ComputerError, ComputerInput, ListInput, VirtualMachine};

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

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
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
            Tile::Wall => write!(f, "#"),
            Tile::Block => write!(f, "+"),
            Tile::Paddle => write!(f, "-"),
            Tile::Ball => write!(f, "o"),
        }
    }
}

pub enum Command {
    Tile(Pos, Tile),
    Score(i64),
}

pub struct Game {
    _board: HashMap<Pos, Tile>,
}

impl Game {
    pub fn paint_board(code: &Code) -> Result<Game, ComputerError> {
        let vm = VirtualMachine::new(code);
        let mut board = HashMap::new();
        while let Some(Command::Tile(pos, tile)) = Game::get_tile(&vm)? {
            board.insert(pos, tile);
        }

        Ok(Game { _board: board })
    }

    fn get_tile(computer: &VirtualMachine<ListInput>) -> Result<Option<Command>, ComputerError> {
        let mut output = computer.get_output();
        let result = if let Some(result) = output.take_exactly(3)? {
            let x = result[0];
            let y = result[1];

            if x == -1 && y == 0 {
                Some(Command::Score(result[2]))
            } else {
                Some(Command::Tile(Pos::new(x, y), Tile::try_from(result[2])?))
            }
        } else {
            None
        };

        Ok(result)
    }

    pub fn count_type(&self, tile: Tile) -> usize {
        self._board.values().filter(|other| **other == tile).count()
    }

    pub fn free_game(code: &Code) -> Result<i64, ComputerError> {
        let mut vm = VirtualMachine::new(code);
        let input_device = vm.get_input();
        vm.patch_memory(0, 2);

        loop {
            let mut score = 0i64;
            let mut blocks = HashSet::new();
            let mut paddle: Option<i64> = None;
            while let Some(command) = Game::get_tile(&mut vm)? {
                match command {
                    Command::Score(_score) => {
                        score = _score;
                    }
                    Command::Tile(pos, Tile::Block) => {
                        blocks.insert(pos);
                    }
                    Command::Tile(pos, Tile::Empty) => {
                        blocks.remove(&pos);
                    }
                    Command::Tile(pos, Tile::Paddle) => {
                        paddle = Some(pos.x());
                    }
                    Command::Tile(pos, Tile::Ball) => {
                        let direction = paddle
                            .map(|paddle| (pos.x() - paddle).signum())
                            .unwrap_or(0);
                        input_device.provide_input(direction);
                    }
                    Command::Tile(_, Tile::Wall) => (),
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
        for row in area.rows(true) {
            for col in row.cols(true) {
                let tile = self._board.get(&col).copied().unwrap_or_default();
                write!(f, "{}", tile)?;
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}
