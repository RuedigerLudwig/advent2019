use std::collections::HashMap;

use crate::orbit_error::OrbitError;

pub struct System {
    pub name: String,
    subsystems: Vec<System>,
}

impl System {
    pub fn parse(input: &Vec<String>) -> Result<System, OrbitError> {
        let map = System::to_map(&input)?;
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
        if let Some(result) = self.get_length(from, to, 1).2 {
            Ok(result)
        } else {
            Err(OrbitError::NoPathError(
                String::from(from),
                String::from(to),
            ))
        }
    }

    fn get_length(
        &self,
        from: &str,
        to: &str,
        distance: i32,
    ) -> (Option<i32>, Option<i32>, Option<i32>) {
        let (mut found_from, mut found_to) = if self.name == from {
            (Some(distance), None)
        } else if self.name == to {
            (None, Some(distance))
        } else {
            (None, None)
        };

        for sub in &self.subsystems {
            let (maybe_from, maybe_to, maybe_sum) = sub.get_length(from, to, distance + 1);
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

    fn to_map(input: &Vec<String>) -> Result<HashMap<String, Vec<String>>, OrbitError> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for line in input {
            let parts: Vec<&str> = line.split(")").collect();
            if parts.len() != 2 {
                Err(OrbitError::OnlyTwoPerLine)?
            }
            let center = String::from(parts[0]);
            let orbiter = String::from(parts[1]);
            if let Some(orbits) = map.get_mut(&center) {
                orbits.push(orbiter);
            } else {
                map.insert(center, vec![orbiter]);
            }
        }
        Ok(map)
    }

    fn find_center(map: &HashMap<String, Vec<String>>) -> Result<String, OrbitError> {
        let mut result = Vec::new();
        for maybe_center in map.keys() {
            let mut is_orbiting = false;
            for orbits in map.values() {
                if orbits.contains(maybe_center) {
                    is_orbiting = true;
                    break;
                }
            }
            if !is_orbiting {
                result.push(maybe_center);
            }
        }

        if result.len() != 1 {
            Err(OrbitError::NoCenterFound)
        } else {
            Ok(result[0].clone())
        }
    }

    fn build_system(
        current: &str,
        map: &HashMap<String, Vec<String>>,
    ) -> Result<System, OrbitError> {
        if let Some(orbits) = map.get(current) {
            let subsystems = orbits
                .iter()
                .map(|orbiter| System::build_system(orbiter, map))
                .collect::<Result<_, _>>()?;
            Ok(System {
                name: String::from(current),
                subsystems,
            })
        } else {
            Ok(System {
                name: String::from(current),
                subsystems: vec![],
            })
        }
    }
}
