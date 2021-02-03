use common::{hashset, Direction, Pos as RawPos};
use std::collections::HashSet;

type Pos = RawPos<i32>;

#[derive(Debug, PartialEq)]
pub struct ErisRecursive {
    _map: HashSet<(Pos, i32)>,
    _min_level: i32,
    _max_level: i32,
}

impl ErisRecursive {
    pub fn parse<T: AsRef<str>>(lines: &[T]) -> ErisRecursive {
        let mut map = HashSet::new();
        for (row, line) in (0..).zip(lines) {
            for (col, ch) in (0..).zip(line.as_ref().chars()) {
                if ch != '.' {
                    map.insert((Pos::new(col, row), 0));
                }
            }
        }
        ErisRecursive {
            _map: map,
            _min_level: 0,
            _max_level: 0,
        }
    }

    fn get_neighbors(&self, pos: &Pos, level: i32, direction: Direction) -> HashSet<(Pos, i32)> {
        match direction {
            Direction::East => match (pos.x(), pos.y()) {
                (4, _) => hashset! { (Pos::new(3, 2), level+1) },
                (1, 2) => (0..5)
                    .map(|row| (Pos::new(0, row), level - 1))
                    .collect::<HashSet<_>>(),
                (col, row) => hashset! { (Pos::new(col+1, row), level)},
            },
            Direction::North => match (pos.x(), pos.y()) {
                (_, 0) => hashset! { (Pos::new(2, 1), level+1) },
                (2, 3) => (0..5)
                    .map(|col| (Pos::new(col, 4), level - 1))
                    .collect::<HashSet<_>>(),
                (col, row) => hashset! { (Pos::new(col, row-1), level)},
            },
            Direction::West => match (pos.x(), pos.y()) {
                (0, _) => hashset! { (Pos::new(1, 2), level+1) },
                (3, 2) => (0..5)
                    .map(|row| (Pos::new(4, row), level - 1))
                    .collect::<HashSet<_>>(),
                (col, row) => hashset! { (Pos::new(col-1, row), level)},
            },
            Direction::South => match (pos.x(), pos.y()) {
                (_, 4) => hashset! { (Pos::new(2, 3), level+1) },
                (2, 1) => (0..5)
                    .map(|col| (Pos::new(col, 0), level - 1))
                    .collect::<HashSet<_>>(),
                (col, row) => hashset! { (Pos::new(col, row+1), level)},
            },
        }
    }

    fn count_neighbors(&self, pos: &Pos, level: i32) -> usize {
        let mut count = 0;
        let mut direction = Direction::East;
        for _ in 0..4 {
            count += self
                ._map
                .intersection(&self.get_neighbors(pos, level, direction))
                .count();
            direction = direction.turn_left()
        }
        count
    }

    fn step_recursive(&self) -> ErisRecursive {
        let mut map = HashSet::new();
        let mut max_level = self._max_level + 1;
        let mut min_level = self._min_level - 1;
        for col in 0..5 {
            for row in 0..5 {
                if col == 2 && row == 2 {
                    continue;
                }

                let pos = Pos::new(col, row);
                for level in self._min_level - 1..=self._max_level + 1 {
                    let bug_is_here = match self.count_neighbors(&pos, level) {
                        1 => true,
                        2 => !self._map.contains(&(pos, level)),
                        _ => false,
                    };

                    if bug_is_here {
                        map.insert((pos, level));

                        if level == self._max_level + 1
                            && (row == 0 || row == 4 || col == 0 || col == 4)
                        {
                            max_level += 1;
                        } else if level == self._min_level - 1
                            && ((col == 2 && (row == 1 || (row == 3)))
                                || (row == 2 && (col == 1 || (col == 3))))
                        {
                            min_level -= 1;
                        }
                    }
                }
            }
        }
        ErisRecursive {
            _map: map,
            _max_level: max_level,
            _min_level: min_level,
        }
    }

    pub fn repeat(&self, times: i32) -> ErisRecursive {
        let mut run = self.step_recursive();
        for _ in 1..times {
            run = run.step_recursive()
        }
        run
    }

    pub fn count_bugs(&self) -> usize {
        self._map.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::{error::CommonError, read_all_lines};

    #[test]
    fn test_one() -> Result<(), CommonError> {
        let input = ErisRecursive::parse(&read_all_lines("day24", "example1.txt")?);
        let expected = 99;

        let result = input.repeat(10).count_bugs();

        assert_eq!(expected, result);

        Ok(())
    }
}
