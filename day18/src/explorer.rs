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
}

impl<'a> Explorer<'a> {
    pub fn new(map: &'a Map) -> Explorer<'a> {
        Explorer {
            _map: map,
            _explored: HashSet::new(),
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

    fn check_and_prepare_special(&mut self, start: Pos) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                if (x != 1 || y != 1) && self._map.get_tile(start + Pos::new(x - 1, y - 1)) != Floor
                {
                    return false;
                }
            }
        }

        for i in 0..2 {
            self._explored.insert(start + Pos::new(1 - 2 * i, 0));
            self._explored.insert(start + Pos::new(0, 1 - 2 * i));
        }
        true
    }

    fn get_special_start_tile(&mut self, start: Pos) -> Option<(Pos, usize)> {
        let relative_start = vec![
            Pos::new(1, 1),
            Pos::new(1, -1),
            Pos::new(-1, -1),
            Pos::new(-1, 1),
        ];

        for corner in relative_start {
            let actual_start = start + corner;
            if !self._explored.contains(&actual_start) {
                self._explored.insert(actual_start);
                return Some((actual_start, 2));
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

    fn next_tile(&mut self, start: Pos, is_special: bool) -> Option<(Pos, usize)> {
        if is_special {
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
        is_special: bool,
    ) -> Option<Path> {
        let mut result = Vec::new();
        while let Some((next_position, steps)) = self.next_tile(start, is_special) {
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
            _ => Some(Path::merge(result, is_special)),
        }
    }

    pub fn explore(&mut self) -> Result<Path, VaultError> {
        let start = self._map.get_entrance()?;
        self._explored.insert(start);
        let is_special = self.check_and_prepare_special(start);

        if let Some(path) = self.go_deeper(start, &HashSet::new(), is_special) {
            Ok(path)
        } else {
            Err(VaultError::NoPath)
        }
    }

    pub fn explore2(&mut self) -> Result<Vec<Path>, VaultError> {
        let start = self._map.get_entrance()?;
        self._explored.insert(start);
        self.check_and_prepare_special(start);

        let relative_start = vec![
            Pos::new(1, 1),
            Pos::new(1, -1),
            Pos::new(-1, -1),
            Pos::new(-1, 1),
        ];

        let paths = relative_start
            .iter()
            .map(|corner| {
                let actual_start = start + *corner;
                self._explored.insert(actual_start);
                self.go_deeper(actual_start, &HashSet::new(), false)
            })
            .collect::<Option<Vec<_>>>();

        if let Some(paths) = paths {
            Ok(paths)
        } else {
            Err(VaultError::NoPath)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::read_all_lines;
    use std::error::Error;

    #[test]
    #[ignore]
    fn test_print_explored() -> Result<(), Box<dyn Error>> {
        let input = read_all_lines("day18", "example5.txt")?;
        let map = Map::new(&input)?;
        let path = Explorer::new(&map).explore()?;
        println!("{}", path);

        Ok(())
    }
}
