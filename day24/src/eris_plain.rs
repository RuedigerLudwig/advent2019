use common::{direction::Direction, pos::Pos as RawPos};
use std::collections::HashSet;

type Pos = RawPos<i32>;

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

    fn count_neighbors(&self, pos: &Pos) -> usize {
        let mut count = 0;
        let mut direction = Direction::East;
        for _ in 0..4 {
            let next_pos = pos + direction;
            if self.map.contains(&next_pos) {
                count += 1;
            }
            direction = direction.turn_left()
        }
        count
    }

    pub fn step(&self) -> Eris {
        let mut map = HashSet::new();
        for col in 0..5 {
            for row in 0..5 {
                let pos = Pos::new(col, row);
                let bug_is_here = match self.count_neighbors(&pos) {
                    1 => true,
                    2 => !self.map.contains(&pos),
                    _ => false,
                };
                if bug_is_here {
                    map.insert(pos);
                }
            }
        }
        Eris { map }
    }

    pub fn rate(&self) -> i64 {
        let mut result = 0i64;
        let mut power = 1i64;
        for row in 0..5 {
            for col in 0..5 {
                if self.map.contains(&Pos::new(col, row)) {
                    result += power;
                }
                power *= 2;
            }
        }
        result
    }

    pub fn run_till_stable(&self) -> Eris {
        let mut ratings = vec![self.rate()];
        let mut next_eris = self.step();
        let mut rating = next_eris.rate();
        while !ratings.contains(&rating) {
            ratings.push(rating);
            next_eris = next_eris.step();
            rating = next_eris.rate();
        }
        next_eris
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::file::read_data;

    #[test]
    fn test_one() -> Result<(), std::io::Error> {
        let input = Eris::parse(&read_data("day24", "example1.txt")?);
        let expected = Eris::parse(&read_data("day24", "expected11.txt")?);

        let result = input.step();

        assert_eq!(expected, result);

        Ok(())
    }

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
