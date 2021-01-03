use common::read_all_lines;
mod orbit_error;
mod orbits;

use orbit_error::OrbitError;
use orbits::System;

pub fn result1() -> Result<String, OrbitError> {
    let input = read_all_lines("day06", "input.txt")?;
    let system = System::parse(&input)?;

    Ok(format!("Day 06 - Result 1: {}", system.count_orbits()))
}

pub fn result2() -> Result<String, OrbitError> {
    let input = read_all_lines("day06", "input.txt")?;
    let system = System::parse(&input)?;

    let result = system.count_transfers("YOU", "SAN")?;
    Ok(format!("Day 06 - Result 2: {}", result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() -> Result<(), OrbitError> {
        let input = read_all_lines("day06", "example1.txt")?;
        let system = System::parse(&input)?;
        let expected = "COM";
        assert_eq!(system.name, expected);

        Ok(())
    }

    #[test]
    fn path_length() -> Result<(), OrbitError> {
        let input = read_all_lines("day06", "example1.txt")?;
        let system = System::parse(&input)?;
        let expected = 42;
        assert_eq!(system.count_orbits(), expected);

        Ok(())
    }

    #[test]
    fn transfers_required() -> Result<(), OrbitError> {
        let input = read_all_lines("day06", "example2.txt")?;
        let system = System::parse(&input)?;
        let expected = 4;
        let result = system.count_transfers("YOU", "SAN")?;
        assert_eq!(result, expected);

        Ok(())
    }
}
