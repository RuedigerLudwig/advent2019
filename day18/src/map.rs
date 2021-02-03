use crate::error::VaultError;
use common::{Area as RawArea, Pos as RawPos};
use std::{collections::HashMap, convert::TryFrom, convert::TryInto, fmt::Display};

pub const ENTRANCE: char = '@';

type Pos = RawPos<i32>;
type Area = RawArea<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Wall,
    Floor,
    Entrance,
    Door(char),
    Key(char),
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Wall
    }
}

impl TryFrom<char> for Tile {
    type Error = VaultError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Floor),
            '@' => Ok(Tile::Entrance),
            'a'..='z' => Ok(Tile::Key(ch)),
            'A'..='Z' => Ok(Tile::Door(ch.to_ascii_lowercase())),
            _ => Err(VaultError::UnknownTile(ch)),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Tile::Wall => write!(f, "#"),
            Tile::Floor => write!(f, "."),
            Tile::Entrance => write!(f, "@"),
            Tile::Key(ch) => write!(f, "{}", ch),
            Tile::Door(ch) => write!(f, "{}", ch.to_ascii_uppercase()),
        }
    }
}

pub struct Map {
    _data: HashMap<Pos, Tile>,
}

impl Map {
    pub fn new<T: AsRef<str>>(lines: &[T]) -> Result<Map, VaultError> {
        let mut _data = HashMap::new();
        for (row, line) in (0..).zip(lines.iter().rev()) {
            for (col, ch) in (0..).zip(line.as_ref().chars()) {
                let tile: Tile = ch.try_into()?;
                if tile != Tile::Wall {
                    _data.insert(Pos::new(col, row), tile);
                }
            }
        }
        Ok(Map { _data })
    }

    pub fn get_tile(&self, pos: Pos) -> Tile {
        self._data.get(&pos).copied().unwrap_or_default()
    }

    pub fn get_entrance(&self) -> Result<Pos, VaultError> {
        let maybe = self
            ._data
            .iter()
            .filter(|(_, tile)| matches!(tile, Tile::Entrance))
            .collect::<Vec<_>>();

        if maybe.len() != 1 {
            Err(VaultError::ExactlyOneEntrance)
        } else {
            Ok(*maybe[0].0)
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let area = self._data.keys().copied().collect::<Area>();
        let upper = "#".repeat((area.width() + 2) as usize);

        writeln!(f, "{}", upper)?;
        for row in area.rows(false) {
            write!(f, "#")?;
            for cell in row.cols(true) {
                let tile = self.get_tile(cell);
                write!(f, "{}", tile)?;
            }
            writeln!(f, "#")?;
        }
        writeln!(f, "{}", upper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_all_lines;

    #[test]
    fn test_get_entrance() -> Result<(), VaultError> {
        let input = read_all_lines("day18", "example1.txt")?;
        let map = Map::new(&input)?;
        let expected = Pos::new(5, 1);
        let result = map.get_entrance()?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_print_path() -> Result<(), VaultError> {
        let input = read_all_lines("day18", "input.txt")?;
        let map = Map::new(&input)?;

        println!("{}", map);

        Ok(())
    }
}
