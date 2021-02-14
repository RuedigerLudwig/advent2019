use crate::common::{area::Area as RawArea, direction::Direction, pos::Pos as RawPos, turn::Turn};
use std::collections::HashSet;

type Pos = RawPos<i32>;
type Area = RawArea<i32>;

#[derive(Debug, PartialEq)]
pub struct Eris {
    map: HashSet<Pos>,
}

impl Eris {
    pub fn parse(input: &str) -> Eris {
        let mut map = HashSet::new();
        for (row, line) in (0..).zip(input.lines()) {
            for (col, ch) in (0..).zip(line.chars()) {
                if ch != '.' {
                    map.insert(Pos::new(col, row));
                }
            }
        }
        Eris { map }
    }

    fn count_neighbors(map: &HashSet<Pos>, pos: &Pos) -> usize {
        let mut count = 0;
        let mut direction = Direction::East;
        for _ in 0..4 {
            let next_pos = pos + direction;
            if map.contains(&next_pos) {
                count += 1;
            }
            direction = direction + Turn::Left;
        }
        count
    }

    pub fn run_till_stable(&self) -> Eris {
        let mut ratings = vec![self.rate()];
        let area = Area::new(Pos::new(0, 0), Pos::new(4, 4));
        let mut map = self.map.clone();

        loop {
            map = area
                .cells(true)
                .filter(|pos| match Eris::count_neighbors(&map, &pos) {
                    1 => true,
                    2 => !map.contains(&pos),
                    _ => false,
                })
                .collect();

            let rating = Eris::rate_map(&map);
            if ratings.contains(&rating) {
                break;
            }

            ratings.push(rating);
        }

        Eris { map }
    }

    fn rate_map(map: &HashSet<Pos>) -> i64 {
        let mut result = 0i64;
        let mut power = 1i64;

        let area = Area::new(Pos::new(0, 0), Pos::new(4, 4));
        for pos in area.cells(true) {
            if map.contains(&pos) {
                result += power;
            }
            power *= 2;
        }
        result
    }

    pub fn rate(&self) -> i64 {
        Eris::rate_map(&self.map)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::file::read_data;

    #[test]
    fn test_biodiversity() -> Result<(), std::io::Error> {
        let input = Eris::parse(&read_data("day24", "stable1.txt")?);
        let expected = 2129920;

        let result = input.rate();

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_run() -> Result<(), std::io::Error> {
        let input = Eris::parse(&read_data("day24", "example1.txt")?);
        let expected = Eris::parse(&read_data("day24", "stable1.txt")?);

        let result = input.run_till_stable();

        assert_eq!(expected, result);

        Ok(())
    }
}
