use std::collections::HashMap;

use crate::error::OrbitError;

pub struct System<'a> {
    pub name: &'a str,
    subsystems: Vec<System<'a>>,
}

impl<'a> System<'a> {
    pub fn parse<T: AsRef<str>>(input: &[T]) -> Result<System<'_>, OrbitError> {
        let map = System::to_map(input)?;
        let center = System::find_center(&map)?;

        System::build_system(&center, &map)
    }

    pub fn count_orbits(&self) -> i32 {
        self.calc_sum_orbits(1)
    }

    fn calc_sum_orbits(&self, path_len: i32) -> i32 {
        self.subsystems
            .iter()
            .map(|s| path_len + s.calc_sum_orbits(path_len + 1))
            .sum()
    }

    pub fn count_transfers(&self, from: &str, to: &str) -> Result<i32, OrbitError> {
        if let Some(result) = self.get_distance(from, to, 1).2 {
            Ok(result)
        } else {
            Err(OrbitError::NoPathError(from.to_owned(), to.to_owned()))
        }
    }

    fn get_distance(
        &self,
        from: &str,
        to: &str,
        distance: i32,
    ) -> (Option<i32>, Option<i32>, Option<i32>) {
        let (mut found_from, mut found_to) = match self.name {
            name if name == from => (Some(distance), None),
            name if name == to => (None, Some(distance)),
            _ => (None, None),
        };

        for sub in &self.subsystems {
            let (maybe_from, maybe_to, maybe_sum) = sub.get_distance(from, to, distance + 1);
            if maybe_sum.is_some() {
                return (None, None, maybe_sum);
            }
            found_from = found_from.or(maybe_from);
            found_to = found_to.or(maybe_to);
            if let Some(sum) = found_from
                .zip(found_to)
                .map(|(a, b)| a + b - distance * 2 - 2)
            {
                return (None, None, Some(sum));
            }
        }

        (found_from, found_to, None)
    }

    fn to_map<T: AsRef<str>>(input: &'a [T]) -> Result<HashMap<&'a str, Vec<&'a str>>, OrbitError> {
        let mut map: HashMap<_, Vec<&'a str>> = HashMap::new();
        for line in input {
            let mut parts = line.as_ref().split(")");

            let (center, orbiter) = if let Some((c, o)) = parts.next().zip(parts.next()) {
                (c, o)
            } else {
                return Err(OrbitError::OnlyTwoPerLine);
            };

            let orbits = map.entry(center).or_default();
            orbits.push(orbiter);
        }
        Ok(map)
    }

    fn find_center(map: &HashMap<&'a str, Vec<&'a str>>) -> Result<&'a str, OrbitError> {
        let mut center = None;
        for maybe_center in map.keys() {
            let mut is_orbiting = false;
            for orbits in map.values() {
                if orbits.contains(maybe_center) {
                    is_orbiting = true;
                    break;
                }
            }
            if !is_orbiting {
                if center.is_none() {
                    center = Some(*maybe_center);
                } else {
                    return Err(OrbitError::NoCenterFound);
                }
            }
        }

        if let Some(center) = center {
            Ok(center)
        } else {
            Err(OrbitError::NoCenterFound)
        }
    }

    fn build_system(
        current: &'a str,
        map: &HashMap<&'a str, Vec<&'a str>>,
    ) -> Result<System<'a>, OrbitError> {
        if let Some(orbits) = map.get(current) {
            let subsystems = orbits
                .iter()
                .map(|orbiter| System::build_system(orbiter, map))
                .collect::<Result<_, _>>()?;
            Ok(System {
                name: current,
                subsystems,
            })
        } else {
            Ok(System {
                name: current,
                subsystems: vec![],
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use common::read_all_lines;

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
