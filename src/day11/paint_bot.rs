use super::{
    error::PaintError,
    interface::{BotComputerInterface, Color},
};
use crate::common::{area::Area as RawArea, direction::Direction, pos::Pos as RawPos};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

type Pos = RawPos<i32>;
type Area = RawArea<i32>;

#[derive(Debug)]
pub struct Bot<T> {
    interface: T,
    board: HashMap<Pos, Color>,
    position: Pos,
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

    pub fn run(&mut self, start_color: Color) -> Result<(), PaintError> {
        self.board.insert(self.position, start_color);
        let mut color = start_color;
        while let Some((paint_color, turn)) = self.interface.accept_input(color)? {
            self.board.insert(self.position, paint_color);
            self.facing = self.facing + turn;
            self.position = self.position + self.facing.as_pos();
            color = self.board.get(&self.position).copied().unwrap_or_default();
        }

        Ok(())
    }
}

impl<T> Bot<T> {
    pub fn count_painted_boards(&self) -> usize {
        self.board.len()
    }
}

impl<T> Display for Bot<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(extent) = Area::from_iterator(self.board.keys()) {
            for row in extent.rows(false) {
                for pos in row.cols(true) {
                    write!(f, "{}", self.board.get(&pos).copied().unwrap_or_default())?;
                }
                writeln!(f, "")?;
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;
    use crate::common::turn::Turn;

    #[test]
    fn test_dummy_paint() -> Result<(), PaintError> {
        let instruction = vec![(1, 0), (0, 0), (1, 0), (1, 0), (0, 1), (1, 0), (1, 0)];
        let interface = TestInterface::new(instruction);
        let mut bot = Bot::new(interface);
        bot.run(Color::Black)?;
        let expected = 6;
        assert_eq!(bot.board.len(), expected);

        Ok(())
    }

    pub struct TestInterface {
        list: Vec<(i64, i64)>,
        index: usize,
    }

    impl TestInterface {
        pub fn new(list: Vec<(i64, i64)>) -> TestInterface {
            TestInterface { list, index: 0 }
        }
    }

    impl BotComputerInterface for TestInterface {
        fn accept_input(&mut self, _color: Color) -> Result<Option<(Color, Turn)>, PaintError> {
            if self.index < self.list.len() {
                let (paint, turn) = self.list[self.index];
                self.index += 1;
                Ok(Some((
                    paint.try_into()?,
                    if turn == 1 { Turn::Right } else { Turn::Left },
                )))
            } else {
                Ok(None)
            }
        }
    }
}
