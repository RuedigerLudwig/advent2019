use std::collections::HashMap;

use crate::error::OrbitError;

pub struct System<'a> {
    pub name: &'a str,
    subsystems: Vec<System<'a>>,
}

impl<'a> System<'a> {
    pub fn parse(input: &str) -> Result<System<'_>, OrbitError> {
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
        self.get_distance(from, to, 1)
            .2
            .ok_or_else(|| OrbitError::NoPathError(from.to_owned(), to.to_owned()))
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

    fn to_map(input: &'a str) -> Result<HashMap<&'a str, Vec<&'a str>>, OrbitError> {
        let mut map: HashMap<_, Vec<&'a str>> = HashMap::new();
        for line in input.lines() {
            let mut parts = line.split(")");

            let (center, orbiter) = parts
                .next()
                .zip(parts.next())
                .ok_or(OrbitError::OnlyTwoPerLine)?;

            let orbits = map.entry(center).or_default();
            orbits.push(orbiter);
        }
        Ok(map)
    }

    fn find_center(map: &HashMap<&'a str, Vec<&'a str>>) -> Result<&'a str, OrbitError> {
        let mut center = None;
        for maybe_center in map.keys() {
            let mut orbiting_something = false;
            for orbits in map.values() {
                if orbits.contains(maybe_center) {
                    orbiting_something = true;
                    break;
                }
            }
            if !orbiting_something {
                if center.is_none() {
                    center = Some(*maybe_center);
                } else {
                    return Err(OrbitError::NoCenterFound);
                }
            }
        }

        center.ok_or(OrbitError::NoCenterFound)
    }

    fn build_system(
        name: &'a str,
        map: &HashMap<&'a str, Vec<&'a str>>,
    ) -> Result<System<'a>, OrbitError> {
        let subsystems = if let Some(orbits) = map.get(name) {
            orbits
                .iter()
                .map(|&orbiter| System::build_system(orbiter, map))
                .collect::<Result<_, _>>()?
        } else {
            vec![]
        };

        Ok(System { name, subsystems })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::file::read_data;

    #[test]
    fn parse() -> Result<(), OrbitError> {
        let input = read_data("day06", "example1.txt")?;
        let system = System::parse(&input)?;
        let expected = "COM";
        assert_eq!(system.name, expected);

        Ok(())
    }

    #[test]
    fn path_length() -> Result<(), OrbitError> {
        let input = read_data("day06", "example1.txt")?;
        let system = System::parse(&input)?;
        let expected = 42;
        assert_eq!(system.count_orbits(), expected);

        Ok(())
    }

    #[test]
    fn transfers_required() -> Result<(), OrbitError> {
        let input = read_data("day06", "example2.txt")?;
        let system = System::parse(&input)?;
        let expected = 4;
        let result = system.count_transfers("YOU", "SAN")?;
        assert_eq!(result, expected);

        Ok(())
    }
}
