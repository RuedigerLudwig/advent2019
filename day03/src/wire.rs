use common::pos::Pos;

use crate::{error::WireError, section::Section};
use std::str::FromStr;

#[derive(Debug)]
pub struct Wire {
    path: Vec<Section>,
}

impl Wire {
    pub fn new(path: &[Section]) -> Wire {
        let mut my_path = Vec::with_capacity(path.len());
        let mut endpoint = Pos::default();
        for section in path {
            let next_section = section.set_start(endpoint);
            endpoint = next_section.end();
            my_path.push(next_section);
        }
        Wire { path: my_path }
    }

    pub fn get_intersections(&self, other: &Wire) -> Vec<(Pos<i32>, i32)> {
        let mut result = Vec::new();
        let mut len1 = 0;
        for sec1 in &self.path {
            let mut len2 = 0;
            for sec2 in &other.path {
                if let Some(crossing) = sec1.intersection(&sec2) {
                    let add = sec1.distance(crossing) + sec2.distance(crossing);
                    result.push((crossing, len1 + len2 + add));
                }
                len2 += sec2.steps();
            }
            len1 += sec1.steps();
        }
        result
    }

    pub fn get_closest_intersection(&self, other: &Wire) -> Option<Pos<i32>> {
        self.get_intersections(other)
            .iter()
            .map(|(p, _)| p)
            .copied()
            .min()
    }

    pub fn get_shortest_intersection(&self, other: &Wire) -> Option<i32> {
        self.get_intersections(other)
            .iter()
            .map(|(_, l)| l)
            .copied()
            .min()
    }
}

impl FromStr for Wire {
    type Err = WireError;

    fn from_str(input: &str) -> Result<Wire, Self::Err> {
        let path = input
            .split(",")
            .map(Section::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Wire::new(&path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{direction::Direction, hashset};
    use std::{collections::HashSet, iter::FromIterator};

    #[test]
    fn test_parse() -> Result<(), WireError> {
        let wire = Wire::from_str(&"U7,R6,D14,L4")?;
        let expected = vec![
            Section::new(Direction::North, 7),
            Section::new(Direction::East, 6).set_start(Pos::new(0, 7)),
            Section::new(Direction::South, 14).set_start(Pos::new(6, 7)),
            Section::new(Direction::West, 4).set_start(Pos::new(6, -7)),
        ];
        assert_eq!(expected, wire.path);

        Ok(())
    }

    #[test]
    fn get_intersections() -> Result<(), WireError> {
        let wire1 = Wire::from_str("R8,U5,L5,D3")?;
        let wire2 = Wire::from_str("U7,R6,D4,L4")?;
        let expected = hashset! {Pos::new(3, 3), Pos::new(6, 5) };
        let crossings = wire1.get_intersections(&wire2);
        let result = HashSet::from_iter(crossings.iter().map(|(p, _)| p).copied());
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn get_closest_intersection() -> Result<(), WireError> {
        let wire1 = Wire::from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72")?;
        let wire2 = Wire::from_str("U62,R66,U55,R34,D71,R55,D58,R83")?;
        let expected = 159;
        if let Some(result) = wire1.get_closest_intersection(&wire2) {
            assert_eq!(expected, result.abs());
            Ok(())
        } else {
            Err(WireError::NoCrossing)
        }
    }

    #[test]
    fn get_shortest_intersection() -> Result<(), WireError> {
        let wire1 = Wire::from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72")?;
        let wire2 = Wire::from_str("U62,R66,U55,R34,D71,R55,D58,R83")?;
        let expected = 610;
        if let Some(result) = wire1.get_shortest_intersection(&wire2) {
            assert_eq!(expected, result);
            Ok(())
        } else {
            Err(WireError::NoCrossing)
        }
    }

    #[test]
    fn get_shortest_intersection2() -> Result<(), WireError> {
        let wire1 = Wire::from_str("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")?;
        let wire2 = Wire::from_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")?;
        let expected = 410;
        if let Some(result) = wire1.get_shortest_intersection(&wire2) {
            assert_eq!(expected, result);
            Ok(())
        } else {
            Err(WireError::NoCrossing)
        }
    }
}
