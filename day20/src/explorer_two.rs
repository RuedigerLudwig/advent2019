use std::{collections::HashMap, fmt::Display};

use crate::{
    error::MapError,
    map::{Map, PortalData},
    paths::Paths,
};

#[derive(Debug)]
pub struct Portal<'a> {
    portal_data: &'a PortalData,
    level: i32,
    steps: i32,
}

impl Display for Portal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}@{})", self.portal_data, self.level)
    }
}

impl<'a> Portal<'a> {
    pub fn get_neighbors(
        &self,
        paths: &'a Paths<'a>,
        safe_guard: &Vec<&'a PortalData>,
    ) -> Result<Vec<Portal<'a>>, MapError> {
        let mut result = vec![];
        let mut did_cut = false;

        if !safe_guard.contains(&self.portal_data) {
            for (portal_data, steps) in paths.get_paths(self.portal_data)? {
                if self.level == 0 {
                    if portal_data.is_outer() && portal_data.is_portal() {
                        did_cut = true;
                        continue;
                    }
                } else if !portal_data.is_portal() {
                    did_cut = true;
                    continue;
                }
                if portal_data.is_entrance() {
                    did_cut = true;
                    continue;
                }

                let next_level = if portal_data.is_outer() {
                    self.level - 1
                } else {
                    self.level + 1
                };

                if portal_data.is_portal() {
                    result.push(Portal {
                        portal_data: paths.get_portal_complement(portal_data),
                        level: next_level,
                        steps: self.steps + steps + 1,
                    })
                } else {
                    result.push(Portal {
                        portal_data,
                        level: next_level,
                        steps: self.steps + steps,
                    })
                }
            }
        }

        match result.len() {
            0 => Ok(result),
            1 => {
                if did_cut || result[0].portal_data.is_exit() {
                    Ok(result)
                } else {
                    let mut safe_guard = safe_guard.clone();
                    safe_guard.push(self.portal_data);
                    result[0].get_neighbors(paths, &safe_guard)
                }
            }
            _ => Ok(result),
        }
    }
}

pub struct ExplorerTwo<'a> {
    paths: Paths<'a>,
}

impl<'a> ExplorerTwo<'a> {
    pub fn new(map: &'a Map) -> ExplorerTwo<'a> {
        ExplorerTwo {
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
            level: 0,
            steps: 0,
        }];

        while let Some(item) = pop_minimum(&mut check_list) {
            let shortest = shortest_paths
                .entry((&item.portal_data.id, item.level))
                .or_insert(i32::MAX);
            if *shortest < item.steps {
                continue;
            } else {
                *shortest = item.steps;
                if item.portal_data.is_exit() {
                    break;
                }
            }

            let neighbors = item.get_neighbors(&self.paths, &vec![])?;
            check_list.extend(neighbors);
        }

        let exit = self.paths.get_exit();
        shortest_paths
            .get(&(&exit.id, -1))
            .copied()
            .ok_or(MapError::NoPath)
    }
}
