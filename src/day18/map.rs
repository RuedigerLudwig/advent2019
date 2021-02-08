use super::error::VaultError;
use crate::common::{area::Area as RawArea, pos::Pos as RawPos};
use std::{collections::HashMap, convert::TryFrom, convert::TryInto, fmt::Display};

pub const ENTRANCE: char = '@';

type Pos = RawPos<i32>;
type Area = RawArea<i32>;

#[derive(Debug, Clone, Copy)]
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
    data: HashMap<Pos, Tile>,
}

impl Map {
    pub fn new(input: &str) -> Result<Map, VaultError> {
        let mut data = HashMap::new();
        for (row, line) in (0..).zip(input.lines().rev()) {
            for (col, ch) in (0..).zip(line.chars()) {
                let tile: Tile = ch.try_into()?;
                if !matches!(tile, Tile::Wall) {
                    data.insert(Pos::new(col, row), tile);
                }
            }
        }
        Ok(Map { data })
    }

    pub fn get_tile(&self, pos: Pos) -> Tile {
        self.data.get(&pos).copied().unwrap_or_default()
    }

    pub fn get_entrance(&self) -> Result<Pos, VaultError> {
        let maybe = self
            .data
            .iter()
            .filter_map(|(&pos, &tile)| {
                if matches!(tile, Tile::Entrance) {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if maybe.len() != 1 {
            Err(VaultError::ExactlyOneEntrance)
        } else {
            Ok(maybe[0])
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let area = self.data.keys().copied().collect::<Area>();
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
    use crate::common::file::read_data;

    #[test]
    fn test_get_entrance() -> Result<(), VaultError> {
        let input = read_data("day18", "example1.txt")?;
        let map = Map::new(&input)?;
        let expected = Pos::new(5, 1);
        let result = map.get_entrance()?;
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_print_path() -> Result<(), VaultError> {
        let input = read_data("day18", "input.txt")?;
        let map = Map::new(&input)?;

        println!("{}", map);

        Ok(())
    }
}
