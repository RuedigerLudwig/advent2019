use super::{
    error::TractorError,
    interface::{Pos, TractorInterface},
};
use std::collections::HashMap;

pub struct Tractor<I> {
    interface: I,
}

impl<I> Tractor<I>
where
    I: TractorInterface,
{
    pub fn new(interface: I) -> Tractor<I> {
        Tractor { interface }
    }

    pub fn scan(&mut self, scan_range: i64) -> Result<i64, TractorError> {
        let mut count = 0;
        let mut tractor_start = 0;
        let mut tractor_end = 0;
        for row in 0..scan_range {
            let mut in_tractor_cone = false;
            for col in tractor_start..scan_range {
                if self.interface.check_pull(Pos::new(col, row))? {
                    in_tractor_cone = true;
                    tractor_start = col;
                    break;
                }
            }
            if in_tractor_cone {
                tractor_end = tractor_end.max(tractor_start + 1);
                for col in tractor_end.. {
                    if !self.interface.check_pull(Pos::new(col, row))? {
                        tractor_end = col;
                        break;
                    }
                }
                count += tractor_end.min(scan_range) - tractor_start;
            }
        }

        Ok(count)
    }

    pub fn fit(&mut self, to_fit: i64) -> Result<i64, TractorError> {
        let mut map = HashMap::new();

        let mut tractor_start = 0;
        let mut tractor_end = 0;
        for row in 0.. {
            let mut in_tractor_cone = false;
            for col in tractor_start..tractor_start + to_fit {
                if self.interface.check_pull(Pos::new(col, row))? {
                    in_tractor_cone = true;
                    tractor_start = col;
                    break;
                }
            }
            if in_tractor_cone {
                tractor_end = tractor_end.max(tractor_start + 1);
                for col in tractor_end.. {
                    if !self.interface.check_pull(Pos::new(col, row))? {
                        tractor_end = col;
                        break;
                    }
                }
                map.insert(row, (tractor_start, tractor_end - 1));
                let width = tractor_end - tractor_start;
                if width >= to_fit {
                    let opposite_col = tractor_start + to_fit - 1;
                    let opposite_row = row - to_fit + 1;
                    let width_range = map[&opposite_row];
                    if width_range.0 <= opposite_col && opposite_col <= width_range.1 {
                        return Ok(tractor_start * 10_000 + opposite_row);
                    }
                }
            }
        }

        Err(TractorError::NoData)?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::file::read_data;
    use std::collections::HashSet;

    #[test]
    fn test_scan() -> Result<(), TractorError> {
        let lines = read_data("day19", "example1.txt")?;
        let interface = TestInterface::new(&lines);
        let mut droid = Tractor::new(interface);
        let result = droid.scan(10)?;
        let expected = 27;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_fit() -> Result<(), TractorError> {
        let lines = read_data("day19", "example1.txt")?;
        let interface = TestInterface::new(&lines);
        let mut droid = Tractor::new(interface);
        let result = droid.fit(10)?;
        let expected = 250020;
        assert_eq!(result, expected);

        Ok(())
    }

    pub struct TestInterface {
        map: HashSet<Pos>,
    }

    impl TestInterface {
        pub fn new(input: &str) -> TestInterface {
            let mut map = HashSet::new();
            for (row, line) in (0..).zip(input.lines()) {
                for (col, ch) in (0..).zip(line.chars()) {
                    if ch != '.' {
                        map.insert(Pos::new(col, row));
                    }
                }
            }
            TestInterface { map }
        }
    }

    impl TractorInterface for TestInterface {
        fn check_pull(&mut self, position: Pos) -> Result<bool, TractorError> {
            Ok(self.map.contains(&position))
        }
    }
}
