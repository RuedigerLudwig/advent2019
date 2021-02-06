use crate::error::GameError;
use common::{area::Area as RawArea, pos::Pos as RawPos};
use computer::{Code, ListInput, NoInput, VirtualMachine};
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    fmt::Display,
};

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
    type Error = GameError;

    fn try_from(tile: i64) -> Result<Self, Self::Error> {
        match tile {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            _ => Err(GameError::NoValidTile(tile)),
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

#[derive(Debug)]
pub enum Command {
    Tile(Pos, Tile),
    Score(i64),
}

#[derive(Debug)]
pub struct Game {
    _board: HashMap<Pos, Tile>,
}

impl Game {
    pub fn paint_board(code: Code) -> Result<Game, GameError> {
        let mut vm = VirtualMachine::new(code, NoInput {});
        let mut board = HashMap::new();
        while let Some(Command::Tile(pos, tile)) = Game::get_tile(&mut vm)? {
            board.insert(pos, tile);
        }

        Ok(Game { _board: board })
    }

    fn get_tile(vm: &mut VirtualMachine<'_>) -> Result<Option<Command>, GameError> {
        let result = if let Some(result) = vm.take_exactly(3)? {
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
        self._board.values().filter(|&other| *other == tile).count()
    }

    pub fn free_game(code: Code) -> Result<i64, GameError> {
        let mut input = ListInput::new();
        let mut vm = VirtualMachine::new(code, input.clone());
        vm.patch_memory(0, 2);

        loop {
            let mut blocks = HashSet::new();
            let mut paddle = None;
            let mut score = None;
            while let Some(command) = Game::get_tile(&mut vm)? {
                match command {
                    Command::Score(next_score) => {
                        score = Some(next_score);
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
                        input.provide_input(direction);
                    }
                    Command::Tile(_, Tile::Wall) => (),
                }
            }
            if blocks.is_empty() {
                return score.ok_or(GameError::NoScore);
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
