use super::{error::ExteriorError, interface::ExteriorInterface, path::Path};
use crate::common::{area::Area as RawArea, direction::Direction, pos::Pos as RawPos, turn::Turn};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

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
    interface: I,
    data: HashMap<Pos, Tile>,
}

impl<I> Exterior<I> {
    fn get_tile(&self, position: Pos) -> Tile {
        *self.data.get(&position).unwrap_or(&Tile::Space)
    }

    fn is_crossing(&self, position: Pos) -> bool {
        self.get_tile(position + Direction::East) == Tile::Scaffold
            && self.get_tile(position + Direction::North) == Tile::Scaffold
            && self.get_tile(position + Direction::West) == Tile::Scaffold
            && self.get_tile(position + Direction::South) == Tile::Scaffold
    }

    fn get_crossings(&self) -> HashSet<Pos> {
        self.data
            .iter()
            .filter_map(|(&position, &tile)| {
                if matches!(tile, Tile::Scaffold) && self.is_crossing(position) {
                    Some(position)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_alignment(&self) -> i32 {
        self.get_crossings()
            .iter()
            .map(|&position| position.x() * position.y())
            .sum::<i32>()
            .abs()
    }

    fn find_robot(&self) -> Option<(Pos, Direction)> {
        self.data.iter().find_map(|(pos, tile)| {
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
                        result.push(Path::new(last_turn, steps)?);
                        return Ok(result);
                    }
                    turn @ Turn::Left | turn @ Turn::Right => {
                        result.push(Path::new(last_turn, steps)?);
                        last_turn = turn;
                        steps = 1;
                        facing = facing + turn;
                        position = position + facing
                    }
                }
            }
        } else {
            Err(ExteriorError::NoRobot)
        }
    }

    fn break_into_parts(&self) -> Result<Vec<String>, ExteriorError> {
        Path::extract_equal_parts(&self.as_path()?, 3)
            .map(|(main, modules)| {
                let mut result = vec![Path::as_string(&main)];
                result.extend(modules.iter().map(|p| Path::as_string(p)));
                result
            })
            .ok_or(ExteriorError::NoPath)
    }
}

impl<I> Exterior<I>
where
    I: ExteriorInterface,
{
    pub fn new(mut interface: I) -> Result<Exterior<I>, ExteriorError> {
        let picture = interface.get_picture()?;
        let mut data = HashMap::new();
        for (row, line) in (0..).zip(picture) {
            for (col, item) in (0..).zip(line.chars()) {
                let tile: Tile = item.into();
                if !matches!(tile, Tile::Space) {
                    data.insert(Pos::new(col, -row), tile);
                }
            }
        }

        Ok(Exterior { interface, data })
    }

    pub fn run_bot(&mut self, run_silent: bool) -> Result<i64, ExteriorError> {
        let mut answers = self.break_into_parts()?;
        if run_silent {
            answers.push(String::from("n"));
        } else {
            answers.push(String::from("y"));
        }
        let result = self.interface.send_data(&answers, run_silent)?;
        Ok(result)
    }
}

impl<I> Display for Exterior<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(area) = Area::from_iterator(self.data.keys()) {
            for row in area.rows(false) {
                for cell in row.cols(true) {
                    let tile = self.get_tile(cell);
                    write!(f, "{}", tile)?;
                }
                writeln!(f, "")?;
            }
        }
        writeln!(f, "--")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{common::file::read_data, hashset};

    struct TestInterface {
        picture: Vec<String>,
    }

    impl TestInterface {
        pub fn new(module: &str, file: &str) -> Result<TestInterface, ExteriorError> {
            let data = read_data(module, file)?;
            let picture = data.lines().map(String::from).collect::<Vec<_>>();
            Ok(TestInterface { picture })
        }
    }

    impl ExteriorInterface for TestInterface {
        fn get_picture(&mut self) -> Result<Vec<String>, ExteriorError> {
            Ok(self.picture.clone())
        }

        fn send_data(&mut self, _data: &[String], _run_silent: bool) -> Result<i64, ExteriorError> {
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
