use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use common::{Area, Direction, Pos};

use crate::interface::BotComputerInterface;

type IPos = Pos<i32>;
type IArea = Area<i32>;
pub struct Bot<T> {
    interface: T,
    board: HashMap<IPos, bool>,
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

    pub fn run(&mut self) {
        loop {
            let is_white = *self.board.get(&self.position).unwrap_or(&false);
            if let Some((paint_color, turn)) = self.interface.accept_input(is_white) {
                self.board.insert(self.position, paint_color);
                self.facing = self.facing.turn(turn);
                self.position = self.position + self.facing.as_pos();
            } else {
                break;
            }
        }
    }
}

impl<T> Bot<T> {
    pub fn paint_current_board(&mut self, paint_white: bool) {
        self.board.insert(self.position, paint_white);
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
                let is_white = *self.board.get(&pos).unwrap_or(&false);
                write!(f, "{}", if is_white { '#' } else { ' ' })?;
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
    fn test_dummy_paint() {
        let instruction = vec![(1, 0), (0, 0), (1, 0), (1, 0), (0, 1), (1, 0), (1, 0)];
        let interface = TestInterface::new(instruction);
        let mut bot = Bot::new(interface);
        bot.run();
        let expected = 6;
        assert_eq!(bot.board.len(), expected);
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
        fn accept_input(&mut self, _is_white: bool) -> Option<(bool, Turn)> {
            if self.index < self.list.len() {
                let (paint, turn) = self.list[self.index];
                self.index += 1;
                Some((paint == 1, if turn == 1 { Turn::Right } else { Turn::Left }))
            } else {
                None
            }
        }
    }
}
