use std::collections::HashSet;

use common::{Direction, Pos as RawPos};

type Pos = RawPos<i32>;

use Tile::*;

use crate::{
    map::{Map, Tile},
    path::Path,
    vault_error::VaultError,
};

pub struct Explorer<'a> {
    _map: &'a Map,
    _explored: HashSet<Pos>,
    _is_special: bool,
}

impl<'a> Explorer<'a> {
    pub fn new(map: &'a Map) -> Explorer<'a> {
        Explorer {
            _map: map,
            _explored: HashSet::new(),
            _is_special: false,
        }
    }

    fn next_step(&self, position: Pos) -> Option<Pos> {
        let mut face_next = Direction::East;
        for _ in 0..4 {
            let next_pos = position + face_next;
            if !self._explored.contains(&next_pos) {
                return Some(next_pos);
            }
            face_next = face_next.turn_left();
        }
        None
    }

    fn prepare_special(&mut self, start: Pos) {
        for x in 0..3 {
            for y in 0..3 {
                if (x != 1 || y != 1) && self._map.get_tile(start + Pos::new(x - 1, y - 1)) != Floor
                {
                    return;
                }
            }
        }

        self._is_special = true;
        for i in 0..2 {
            self._explored.insert(start + Pos::new(1 - 2 * i, 0));
            self._explored.insert(start + Pos::new(0, 1 - 2 * i));
        }
    }

    fn get_special_start_tile(&mut self, start: Pos) -> Option<(Pos, usize)> {
        for iter in 0..4 {
            let x = 1 - (iter / 2) * 2;
            let y = 1 - (iter % 3i32).signum() * 2;
            let next_pos = start + Pos::new(x, y);
            if !self._explored.contains(&next_pos) {
                self._explored.insert(next_pos);
                return Some((next_pos, 2));
            }
        }
        None
    }

    fn possible_walks(&self, pos: Pos) -> usize {
        let mut result: usize = 0;
        let mut facing = Direction::East;
        for _ in 0..4 {
            let next_pos = pos + facing;
            if !self._explored.contains(&next_pos) && self._map.get_tile(next_pos) != Wall {
                result += 1;
            }
            facing = facing.turn_left();
        }
        result
    }

    fn next_tile(&mut self, start: Pos, at_special_entrance: bool) -> Option<(Pos, usize)> {
        if at_special_entrance {
            self.get_special_start_tile(start)
        } else {
            loop {
                let mut steps: usize = 1;
                let mut position = start;
                while let Some(next_position) = self.next_step(position) {
                    self._explored.insert(next_position);
                    match self._map.get_tile(next_position) {
                        Floor => {
                            if self.possible_walks(next_position) > 1 {
                                return Some((next_position, steps));
                            } else {
                                steps += 1;
                                position = next_position
                            }
                        }
                        Door(_) | Key(_) => return Some((next_position, steps)),
                        Wall | Entrance => {}
                    }
                }
                if position == start {
                    break;
                }
            }
            None
        }
    }

    pub fn go_deeper(
        &mut self,
        start: Pos,
        required: &HashSet<char>,
        at_special_entrance: bool,
    ) -> Option<Path> {
        let mut result = Vec::new();
        while let Some((next_position, steps)) = self.next_tile(start, at_special_entrance) {
            let branch_result = match self._map.get_tile(next_position) {
                Floor => self.go_deeper(next_position, required, false),

                Door(door) => {
                    let mut required = required.clone();
                    required.insert(door);
                    self.go_deeper(next_position, &required, false)
                }

                Key(key) => {
                    let mut result = Path::new();
                    result.add_from_entrance(&key, 0, &required);

                    if let Some(next_map) = self.go_deeper(next_position, required, false) {
                        result.merge_on_key(&key, next_map);
                    }
                    Some(result)
                }

                Tile::Wall | Tile::Entrance => unreachable!(),
            };

            if let Some(mut branch_result) = branch_result {
                branch_result.inc_steps_from_entrance(steps);
                result.push(branch_result);
            }
        }

        match result.len() {
            0 => None,
            1 => Some(result.swap_remove(0)),
            _ => Some(Path::merge(result, at_special_entrance)),
        }
    }

    pub fn explore(&mut self) -> Result<Path, VaultError> {
        let start = self._map.get_entrance()?;
        self._explored.insert(start);
        self.prepare_special(start);

        if let Some(path) = self.go_deeper(start, &HashSet::new(), self._is_special) {
            Ok(path)
        } else {
            Err(VaultError::NoPath)
        }
    }

    pub fn explore2(&mut self) -> Result<Vec<Path>, VaultError> {
        let start = self._map.get_entrance()?;
        self._explored.insert(start);
        self.prepare_special(start);

        let paths = vec![
            Pos::new(1, 1),
            Pos::new(1, -1),
            Pos::new(-1, -1),
            Pos::new(-1, 1),
        ]
        .iter()
        .map(|corner| {
            let next_start = start + *corner;
            self._explored.insert(next_start);
            self.go_deeper(next_start, &HashSet::new(), false)
        })
        .collect::<Option<Vec<_>>>();

        if let Some(paths) = paths {
            Ok(paths)
        } else {
            Err(VaultError::NoPath)
        }
    }
}
