use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use common::{Area as RawArea, Direction, Pos as RawPos, Turn};

use crate::{exterior_error::ExteriorError, interface::ExteriorInterface, path::Path};
type Pos = RawPos<i32>;
type Area = RawArea<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Space,
    Scaffold,
    DeadRobot,
    Robot(Direction),
    Unknown(char),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Tile::Space => write!(f, " "),
            Tile::Scaffold => write!(f, "#"),
            Tile::DeadRobot => write!(f, "X"),
            Tile::Unknown(c) => write!(f, "{}", c),
            Tile::Robot(Direction::East) => write!(f, ">"),
            Tile::Robot(Direction::North) => write!(f, "^"),
            Tile::Robot(Direction::West) => write!(f, "<"),
            Tile::Robot(Direction::South) => write!(f, "v"),
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Scaffold,
            '.' => Tile::Space,
            '>' => Tile::Robot(Direction::East),
            '^' => Tile::Robot(Direction::North),
            '<' => Tile::Robot(Direction::West),
            'v' => Tile::Robot(Direction::South),
            'X' => Tile::DeadRobot,
            c => Tile::Unknown(c),
        }
    }
}

pub struct Exterior<I> {
    _interface: I,
    _data: HashMap<Pos, Tile>,
}

impl<I> Exterior<I> {
    fn get_tile(&self, position: Pos) -> Tile {
        *self._data.get(&position).unwrap_or(&Tile::Space)
    }

    fn is_crossing(&self, position: Pos) -> bool {
        self.get_tile(position + Direction::East) == Tile::Scaffold
            && self.get_tile(position + Direction::North) == Tile::Scaffold
            && self.get_tile(position + Direction::West) == Tile::Scaffold
            && self.get_tile(position + Direction::South) == Tile::Scaffold
    }

    fn get_crossings(&self) -> HashSet<Pos> {
        let mut result = HashSet::new();
        for (position, tile) in &self._data {
            if *tile == Tile::Scaffold && self.is_crossing(*position) {
                result.insert(*position);
            }
        }
        result
    }

    pub fn get_alignment(&self) -> i32 {
        let mut result = 0;
        for position in self.get_crossings() {
            result += position.x() * position.y();
        }
        result.abs()
    }

    fn find_robot(&self) -> Option<(Pos, Direction)> {
        self._data.iter().find_map(|(pos, tile)| {
            if let Tile::Robot(dir) = *tile {
                Some((*pos, dir))
            } else {
                None
            }
        })
    }

    fn best_step(&self, position: Pos, facing: Direction) -> Result<Turn, ExteriorError> {
        if let Tile::Scaffold = self.get_tile(position + facing) {
            Ok(Turn::Forward)
        } else if let Tile::Scaffold = self.get_tile(position + (facing + Turn::Right)) {
            Ok(Turn::Right)
        } else if let Tile::Scaffold = self.get_tile(position + (facing + Turn::Left)) {
            Ok(Turn::Left)
        } else if let Tile::Scaffold = self.get_tile(position + (facing + Turn::Back)) {
            Ok(Turn::Back)
        } else {
            Err(ExteriorError::NoScaffold(position, facing))
        }
    }

    fn as_path(&self) -> Result<Vec<Path>, ExteriorError> {
        if let Some((position, facing)) = self.find_robot() {
            let mut result = Vec::new();
            let mut last_turn = self.best_step(position, facing)?;
            let mut steps = 1;
            let mut facing = facing + last_turn;
            let mut position = position + facing;
            loop {
                match self.best_step(position, facing)? {
                    Turn::Forward => {
                        steps += 1;
                        position = position + facing;
                    }
                    Turn::Back => {
                        result.push(Path::new(last_turn, steps));
                        return Ok(result);
                    }
                    turn @ Turn::Left | turn @ Turn::Right => {
                        result.push(Path::new(last_turn, steps));
                        last_turn = turn;
                        steps = 1;
                        facing = facing.turn(turn);
                        position = position + facing
                    }
                }
            }
        } else {
            Err(ExteriorError::NoRobot)
        }
    }

    fn break_into_parts(&self) -> Result<Vec<String>, ExteriorError> {
        if let Some((main, modules)) = Path::extract_equal_parts(&self.as_path()?, 3) {
            let main = Path::as_string(&main);
            let modules = modules
                .iter()
                .map(|p| Path::as_string(p))
                .collect::<Vec<_>>();

            let mut result = Vec::new();
            result.push(main);
            result.extend(modules);

            Ok(result)
        } else {
            Err(ExteriorError::NoPath)
        }
    }
}

impl<I> Exterior<I>
where
    I: ExteriorInterface,
{
    pub fn new(mut _interface: I) -> Result<Exterior<I>, ExteriorError> {
        let picture = _interface.get_picture()?;
        let mut _data = HashMap::new();
        for (row, line) in (0..).zip(picture.lines()) {
            for (col, item) in (0..).zip(line.chars()) {
                let tile: Tile = item.into();
                if let Tile::Space = tile {
                } else {
                    _data.insert(Pos::new(col, -row), tile);
                }
            }
        }

        Ok(Exterior { _interface, _data })
    }

    pub fn run_bot(&mut self) -> Result<i64, ExteriorError> {
        let data = self.break_into_parts()?;
        let result = self._interface.send_data(&data)?;
        Ok(result)
    }
}

impl<I> Display for Exterior<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let area = self._data.keys().copied().collect::<Area>();
        for row in area.rows(false) {
            for cell in row.cols(true) {
                let tile = self.get_tile(cell);
                write!(f, "{}", tile)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "--")
    }
}

#[cfg(test)]
mod tests {
    use common::{hashset, read_as_string};

    use super::*;

    struct TestInterface {
        data: String,
    }

    impl TestInterface {
        pub fn new(module: &str, file: &str) -> Result<TestInterface, ExteriorError> {
            let data = read_as_string(module, file)?;
            Ok(TestInterface { data })
        }
    }

    impl ExteriorInterface for TestInterface {
        fn get_picture(&mut self) -> Result<String, ExteriorError> {
            Ok(self.data.clone())
        }

        fn send_data(&mut self, _data: &[String]) -> Result<i64, ExteriorError> {
            unimplemented!()
        }
    }

    #[test]
    fn test_scrossings() -> Result<(), ExteriorError> {
        let interface = TestInterface::new("day17", "example1.txt")?;
        let exterior = Exterior::new(interface)?;
        let result = exterior.get_crossings();
        let expected = hashset!(
            Pos::new(2, -2),
            Pos::new(2, -4),
            Pos::new(6, -4),
            Pos::new(10, -4)
        );

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_alignment() -> Result<(), ExteriorError> {
        let interface = TestInterface::new("day17", "example1.txt")?;
        let exterior = Exterior::new(interface)?;
        let result = exterior.get_alignment();
        let expected = 76;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_as_path() -> Result<(), ExteriorError> {
        let interface = TestInterface::new("day17", "example2.txt")?;
        let exterior = Exterior::new(interface)?;
        let path = exterior.as_path()?;
        let result = Path::as_string(&path);
        let expected = "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2";

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_get_parts() -> Result<(), ExteriorError> {
        let interface = TestInterface::new("day17", "example2.txt")?;
        let exterior = Exterior::new(interface)?;
        let path = exterior.break_into_parts();

        assert!(path.is_ok());

        Ok(())
    }
}
