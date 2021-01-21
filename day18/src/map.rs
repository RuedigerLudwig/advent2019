use crate::vault_error::VaultError;
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
    pub fn new(lines: &[String]) -> Result<Map, VaultError> {
        let mut _data = HashMap::new();
        for (row, line) in (0..).zip(lines.iter()) {
            if line.is_empty() {
                break;
            }
            for (col, ch) in (0..).zip(line.chars()) {
                let tile: Tile = ch.try_into()?;
                if let Tile::Wall = tile {
                } else {
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
            .filter(|(_, tile)| {
                if let Tile::Entrance = tile {
                    true
                } else {
                    false
                }
            })
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
    use super::*;
    use common::read_all_lines;
    use std::error::Error;

    #[test]
    fn test_get_entrance() -> Result<(), Box<dyn Error>> {
        let input = read_all_lines("day18", "example1.txt")?;
        let vault = Map::new(&input)?;
        let expected = Pos::new(5, 1);
        let result = vault.get_entrance()?;
        assert_eq!(expected, result);
        Ok(())
    }
}