use super::{
    error::MapError,
    map::{Map, PortalData, Pos, Tile},
};
use crate::{common::direction::Direction, hashmap, hashset};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

#[derive(Debug)]
pub struct Paths<'a> {
    known: RefCell<HashMap<&'a PortalData, HashMap<&'a PortalData, i32>>>,
    map: &'a Map,
}

impl<'a> Paths<'a> {
    pub fn new(map: &'a Map) -> Paths<'_> {
        Paths {
            known: RefCell::new(HashMap::new()),
            map,
        }
    }

    pub fn get_entrance(&self) -> &PortalData {
        self.map.get_entrance()
    }

    pub fn get_exit(&self) -> &PortalData {
        self.map.get_exit()
    }

    pub fn get_portal_complement(&self, portal_data: &PortalData) -> &PortalData {
        self.map.get_portal_complement(portal_data)
    }

    pub fn get_paths(
        &'a self,
        from: &'a PortalData,
    ) -> Result<HashMap<&'a PortalData, i32>, MapError> {
        let mut known = self.known.borrow_mut();
        if let Some(paths) = known.get(&from) {
            Ok(paths.clone())
        } else {
            let explored = Explorer::explore(self.map, &from.position)?;
            known.insert(from, explored);
            Ok(known.get(&from).unwrap().clone())
        }
    }
}

struct Explorer<'a> {
    explored: HashSet<Pos>,
    map: &'a Map,
}

impl<'a> Explorer<'a> {
    pub fn explore(map: &'a Map, start: &Pos) -> Result<HashMap<&'a PortalData, i32>, MapError> {
        Explorer {
            explored: hashset!(*start),
            map,
        }
        .dig_into(start, 0)
    }

    fn walk_to_next_interesting(
        &mut self,
        start: &Pos,
        mut steps: i32,
    ) -> Option<(Tile, Pos, i32)> {
        if self.explored.contains(&start) {
            return None;
        }
        let mut pos = *start;
        loop {
            let tile = *self.map.get_tile(&pos);
            match tile {
                Tile::Floor => {
                    let mut check_facing = Direction::East;
                    let mut free_walks = 0;
                    let mut next_pos = None;
                    for _ in 0..4 {
                        let check_pos = pos + check_facing;
                        if !self.explored.contains(&check_pos) {
                            if let Tile::Wall = self.map.get_tile(&check_pos) {
                            } else {
                                if next_pos.is_none() {
                                    next_pos = Some(check_pos);
                                }
                                free_walks += 1;
                            }
                        }
                        check_facing = check_facing.turn_left();
                    }
                    match free_walks {
                        0 => return None,
                        1 => {
                            self.explored.insert(pos);
                            pos = next_pos.unwrap();
                            steps += 1;
                        }
                        _ => return Some((tile, pos, steps)),
                    }
                }
                Tile::Portal => return Some((tile, pos, steps)),
                Tile::Wall => return None,
            }
        }
    }

    fn next_step(&mut self, pos: &Pos, steps: i32) -> Option<(Tile, Pos, i32)> {
        let mut facing = Direction::East;

        for _ in 0..4 {
            let next_pos = pos + facing;
            if let Some(result) = self.walk_to_next_interesting(&next_pos, steps + 1) {
                return Some(result);
            }
            facing = facing.turn_left();
        }

        None
    }

    fn dig_into(
        &mut self,
        start: &Pos,
        steps: i32,
    ) -> Result<HashMap<&'a PortalData, i32>, MapError> {
        let mut result = hashmap! {};

        while let Some((tile, next_pos, steps)) = self.next_step(start, steps) {
            self.explored.insert(next_pos);
            match tile {
                Tile::Floor => {
                    let following = self.dig_into(&next_pos, steps)?;
                    result.extend(following);
                }

                Tile::Portal => {
                    let portal_data = self.map.get_portal(&next_pos)?;
                    result.insert(portal_data, steps);
                }

                Tile::Wall => {}
            }
        }

        Ok(result)
    }
}
