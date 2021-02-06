use crate::{
    error::VaultError,
    map::{Map, Tile},
    path::Path,
};
use common::{direction::Direction, pos::Pos as RawPos};
use std::collections::HashSet;

type Pos = RawPos<i32>;

use Tile::*;

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
                if (x != 1 || y != 1)
                    && !matches!(self._map.get_tile(start + Pos::new(x - 1, y - 1)), Floor)
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

    fn get_special_start_tile(&mut self, start: Pos) -> Option<(Tile, Pos, usize)> {
        let corners = vec![
            Pos::new(1, 1),
            Pos::new(1, -1),
            Pos::new(-1, -1),
            Pos::new(-1, 1),
        ];

        for corner in corners {
            let actual_start = start + corner;
            if !self._explored.contains(&actual_start) {
                self._explored.insert(actual_start);
                return Some((Tile::Floor, actual_start, 2));
            }
        }
        None
    }

    fn count_possible_exits(&self, pos: Pos) -> usize {
        let mut result: usize = 0;
        let mut facing = Direction::East;
        for _ in 0..4 {
            let next_pos = pos + facing;
            if !self._explored.contains(&next_pos) && !matches!(self._map.get_tile(next_pos), Wall)
            {
                result += 1;
            }
            facing = facing.turn_left();
        }
        result
    }

    fn get_next_tile(&mut self, start: Pos, is_special: bool) -> Option<(Tile, Pos, usize)> {
        if is_special {
            return self.get_special_start_tile(start);
        }

        loop {
            let mut steps: usize = 1;
            let mut position = start;
            while let Some(next_position) = self.next_step(position) {
                self._explored.insert(next_position);
                match self._map.get_tile(next_position) {
                    tile @ Floor => {
                        if self.count_possible_exits(next_position) > 1 {
                            return Some((tile, next_position, steps));
                        } else {
                            steps += 1;
                            position = next_position
                        }
                    }
                    tile @ Door(_) | tile @ Key(_) => return Some((tile, next_position, steps)),
                    Wall => (),
                    Entrance => unimplemented!(),
                }
            }
            if steps == 1 {
                return None;
            }
        }
    }

    pub fn dig_deeper(
        &mut self,
        start: Pos,
        required: &HashSet<char>,
        is_special: bool,
    ) -> Option<Path> {
        let mut result = Vec::new();
        while let Some((tile, next_position, steps)) = self.get_next_tile(start, is_special) {
            let branch_result = match tile {
                Floor => self.dig_deeper(next_position, required, false),

                Door(door) => {
                    let mut required = required.clone();
                    required.insert(door);
                    self.dig_deeper(next_position, &required, false)
                }

                Key(key) => {
                    let mut result = Path::new();
                    result.add_from_entrance(&key, 0, &required);

                    if let Some(next_map) = self.dig_deeper(next_position, required, false) {
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

    pub fn explore_part1(&mut self) -> Result<Path, VaultError> {
        let start = self._map.get_entrance()?;
        self._explored.insert(start);
        let is_special = self.check_and_prepare_special(start);

        self.dig_deeper(start, &HashSet::new(), is_special)
            .ok_or(VaultError::NoPath)
    }

    pub fn explore_part2(&mut self) -> Result<Vec<Path>, VaultError> {
        let start = self._map.get_entrance()?;
        self._explored.insert(start);
        if !(self.check_and_prepare_special(start)) {
            return Err(VaultError::NotSpecial);
        }

        vec![
            Pos::new(1, 1),
            Pos::new(1, -1),
            Pos::new(-1, -1),
            Pos::new(-1, 1),
        ]
        .iter()
        .map(|corner| {
            let actual_start = start + *corner;
            self._explored.insert(actual_start);
            self.dig_deeper(actual_start, &HashSet::new(), false)
        })
        .collect::<Option<Vec<_>>>()
        .ok_or(VaultError::NoPath)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::file::read_data;

    #[test]
    #[ignore]
    fn test_print_explored() -> Result<(), VaultError> {
        let input = read_data("day18", "example5.txt")?;
        let map = Map::new(&input)?;
        let path = Explorer::new(&map).explore_part1()?;
        println!("{}", path);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_print_explored2() -> Result<(), VaultError> {
        let input = read_data("day18", "input.txt")?;
        let map = Map::new(&input)?;
        let paths = Explorer::new(&map).explore_part2()?;
        for path in paths {
            println!("{}", path);
        }

        Ok(())
    }
}
