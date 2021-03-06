use std::collections::{HashMap, HashSet};

use crate::hashset;

use super::{
    error::MapError,
    map::{Map, PortalData},
    paths::Paths,
};

pub struct Portal<'a> {
    portal_data: &'a PortalData,
    steps: i32,
    visited: HashSet<String>,
}

impl<'a> Portal<'a> {
    pub fn get_neighbors(&self, paths: &'a Paths<'a>) -> Result<Vec<Portal<'a>>, MapError> {
        let mut result = vec![];

        let (portal, portal_steps) = match &self.portal_data.id {
            id if id == "AA" => (self.portal_data, 0),
            id if id == "ZZ" => return Ok(vec![]),
            _ => (paths.get_portal_complement(self.portal_data), 1),
        };

        let path = paths.get_paths(portal)?;

        for (portal_data, steps) in path {
            if !self.visited.contains(&portal_data.id) {
                result.push(Portal {
                    portal_data,

                    steps: self.steps + steps + portal_steps,
                    visited: {
                        let mut visited = self.visited.clone();
                        visited.insert(portal_data.id.to_owned());
                        visited
                    },
                })
            }
        }

        Ok(result)
    }
}

pub struct Explorer<'a> {
    paths: Paths<'a>,
}

impl<'a> Explorer<'a> {
    pub fn new(map: &'a Map) -> Explorer<'a> {
        Explorer {
            paths: Paths::new(map),
        }
    }

    pub fn explore(&'a mut self) -> Result<i32, MapError> {
        fn pop_minimum<'a>(list: &mut Vec<Portal<'a>>) -> Option<Portal<'a>> {
            if let Some(min) = list.iter().min_by_key(|c| c.steps) {
                let index = list
                    .iter()
                    .position(|content| content.steps == min.steps)
                    .unwrap_or_default();
                Some(list.swap_remove(index))
            } else {
                None
            }
        }

        let mut shortest_paths = HashMap::new();

        let entrance = self.paths.get_entrance();
        let mut check_list = vec![Portal {
            portal_data: entrance,
            steps: 0,
            visited: hashset!(entrance.id.to_owned()),
        }];
        while let Some(item) = pop_minimum(&mut check_list) {
            let shortest = shortest_paths
                .entry(item.portal_data.id.to_owned())
                .or_insert(i32::MAX);
            if *shortest < item.steps {
                continue;
            } else {
                *shortest = item.steps;
            }
            let neighbors = item.get_neighbors(&self.paths)?;
            check_list.extend(neighbors);
        }

        let exit = self.paths.get_exit();
        shortest_paths
            .get(&exit.id)
            .copied()
            .ok_or(MapError::NoPath)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::file::read_data;

    #[test]
    fn test_explore() -> Result<(), MapError> {
        let input = read_data("day20", "example1.txt")?;
        let map = Map::parse(&input)?;
        let len = Explorer::new(&map).explore()?;
        assert_eq!(23, len);

        Ok(())
    }

    #[test]
    fn test_explore2() -> Result<(), MapError> {
        let input = read_data("day20", "example2.txt")?;
        let map = Map::parse(&input)?;
        let len = Explorer::new(&map).explore()?;
        assert_eq!(58, len);

        Ok(())
    }
}
