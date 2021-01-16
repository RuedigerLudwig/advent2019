use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use common::{Area, Direction, Pos};
use computer::ComputerError;

use crate::interface::{BotComputerInterface, Color};

type IPos = Pos<i32>;
type IArea = Area<i32>;
pub struct Bot<T> {
    interface: T,
    board: HashMap<IPos, Color>,
    position: IPos,
    facing: Direction,
}

impl<T: BotComputerInterface> Bot<T> {
    pub fn new(interface: T) -> Bot<T> {
        Bot {
            interface,
            board: HashMap::new(),
            position: Pos::default(),
            facing: Direction::North,
        }
    }

    pub fn run(&mut self) -> Result<(), ComputerError> {
        loop {
            let color = self.board.get(&self.position).copied().unwrap_or_default();
            if let Some((paint_color, turn)) = self.interface.accept_input(color)? {
                self.board.insert(self.position, paint_color);
                self.facing = self.facing.turn(turn);
                self.position = self.position + self.facing.as_pos();
            } else {
                break;
            }
        }

        Ok(())
    }
}

impl<T> Bot<T> {
    pub fn paint_current_board(&mut self, color: Color) {
        self.board.insert(self.position, color);
    }

    pub fn count_painted_boards(&self) -> usize {
        self.board.len()
    }

    fn get_extend(&self) -> IArea {
        self.board
            .keys()
            .fold(Area::default(), |area, pos| area.extend(*pos))
    }
}

impl<T> Display for Bot<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let extent = self.get_extend();
        for row in extent.rows(false) {
            for pos in row.cols(true) {
                write!(f, "{}", self.board.get(&pos).copied().unwrap_or_default())?;
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use common::Turn;

    use super::*;

    #[test]
    fn test_dummy_paint() -> Result<(), ComputerError> {
        let instruction = vec![(1, 0), (0, 0), (1, 0), (1, 0), (0, 1), (1, 0), (1, 0)];
        let interface = TestInterface::new(instruction);
        let mut bot = Bot::new(interface);
        bot.run()?;
        let expected = 6;
        assert_eq!(bot.board.len(), expected);

        Ok(())
    }

    pub struct TestInterface {
        list: Vec<(i32, i32)>,
        index: usize,
    }

    impl TestInterface {
        pub fn new(list: Vec<(i32, i32)>) -> TestInterface {
            TestInterface { list, index: 0 }
        }
    }

    impl BotComputerInterface for TestInterface {
        fn accept_input(&mut self, _color: Color) -> Result<Option<(Color, Turn)>, ComputerError> {
            if self.index < self.list.len() {
                let (paint, turn) = self.list[self.index];
                self.index += 1;
                Ok(Some((
                    if paint == 1 {
                        Color::White
                    } else {
                        Color::Black
                    },
                    if turn == 1 { Turn::Right } else { Turn::Left },
                )))
            } else {
                Ok(None)
            }
        }
    }
}
