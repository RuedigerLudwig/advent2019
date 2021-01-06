use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use common::{CommonError, Pos};

pub struct Asteroids {
    field: HashSet<Pos<i32>>,
}

impl Asteroids {
    pub fn parse(input: &Vec<String>) -> Asteroids {
        let mut field = HashSet::new();
        for (y, line) in (0..).zip(input.iter()) {
            for (x, place) in (0..).zip(line.chars()) {
                if place == '#' {
                    field.insert(Pos::new(x, -y));
                }
            }
        }
        Asteroids { field }
    }

    pub fn get_visible(&self, center: Pos<i32>) -> HashSet<Pos<i32>> {
        let mut visible_map = HashMap::new();
        for asteroid in &self.field {
            if *asteroid != center {
                let diff = *asteroid - center;
                let (normal, factor) = diff.normalize();
                if let Some(old_factor) = visible_map.get_mut(&normal) {
                    if *old_factor > factor {
                        *old_factor = factor;
                    }
                } else {
                    visible_map.insert(normal, factor);
                }
            }
        }

        let mut result = HashSet::new();
        for (normal, factor) in visible_map {
            result.insert(normal * factor + center);
        }

        result
    }

    pub fn get_best_position(&self) -> Result<(Pos<i32>, usize), CommonError> {
        let mut result = Vec::new();
        let mut max = 0;
        for center in &self.field {
            let visible = self.get_visible(*center).len();

            if visible > max {
                max = visible;
                result = Vec::new();
            }
            if visible == max {
                result.push(*center);
            }
        }

        if result.len() != 1 {
            Err(CommonError::MessageError(String::from(
                "Did not find a best center",
            )))
        } else {
            Ok((*result.get(0).unwrap(), max))
        }
    }

    fn pos_cmp<'a, 'b>(pos1: &'a Pos<i32>, pos2: &'b Pos<i32>) -> Ordering {
        pos1.angle2().partial_cmp(&pos2.angle2()).unwrap()
    }

    pub fn get_lasered_asteroids(&self, center: Pos<i32>) -> Vec<Pos<i32>> {
        let mut visible_map: HashMap<Pos<i32>, Vec<i32>> = HashMap::new();
        for asteroid in &self.field {
            if *asteroid != center {
                let diff = *asteroid - center;
                let (normal, factor) = diff.normalize();
                if let Some(lst) = visible_map.get_mut(&normal) {
                    lst.push(factor);
                    lst.sort_by(|a, b| b.cmp(a));
                } else {
                    let mut lst = Vec::new();
                    lst.push(factor);
                    visible_map.insert(normal, lst);
                }
            }
        }

        let mut sorted_normals = visible_map.keys().copied().collect::<Vec<_>>();
        sorted_normals.sort_by(Asteroids::pos_cmp);

        let mut result = Vec::new();
        loop {
            let mut found_any = false;
            for normal in &sorted_normals {
                if let Some(lst) = visible_map.get_mut(normal) {
                    if let Some(factor) = lst.pop() {
                        result.push(center + *normal * factor);
                        found_any = true;
                    }
                }
            }
            if !found_any {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{hashset, read_all_lines, CommonError};

    #[test]
    fn test_parse_asteroids() -> Result<(), CommonError> {
        let input = read_all_lines("day10", "example1.txt")?;
        let asteroids = Asteroids::parse(&input);
        let expected = 10;
        assert_eq!(asteroids.field.len(), expected);

        Ok(())
    }

    #[test]
    fn test_visible_asteroids() -> Result<(), CommonError> {
        let input = read_all_lines("day10", "example1.txt")?;
        let asteroids = Asteroids::parse(&input);
        let expected = hashset!(
            Pos::new(1, 0),
            Pos::new(4, 0),
            Pos::new(3, -2),
            Pos::new(4, -3),
            Pos::new(3, -4)
        );
        let result = asteroids.get_visible(Pos::new(4, -2));
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_best_position() -> Result<(), CommonError> {
        let input = read_all_lines("day10", "example1.txt")?;
        let asteroids = Asteroids::parse(&input);
        let expected = (Pos::new(3, -4), 8);
        let result = asteroids.get_best_position()?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_best_position2() -> Result<(), CommonError> {
        let input = read_all_lines("day10", "example2.txt")?;
        let asteroids = Asteroids::parse(&input);
        let expected = (Pos::new(5, -8), 33);
        let result = asteroids.get_best_position()?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_laser_order() -> Result<(), CommonError> {
        let input = read_all_lines("day10", "example3.txt")?;
        let asteroids = Asteroids::parse(&input);
        let expected = vec![
            Pos::new(8, -1),
            Pos::new(9, 0),
            Pos::new(9, -1),
            Pos::new(10, 0),
            Pos::new(9, -2),
            Pos::new(11, -1),
            Pos::new(12, -1),
            Pos::new(11, -2),
            Pos::new(15, -1),
        ];
        let result = asteroids.get_lasered_asteroids(Pos::new(8, -3));
        assert_eq!(&result[0..9], expected);
        assert_eq!(result.get(29), Some(&Pos::new(7, 0)));
        assert_eq!(result.get(30), Some(&Pos::new(8, 0)));
        assert_eq!(result.get(34), Some(&Pos::new(13, -3)));
        assert_eq!(result.get(35), Some(&Pos::new(14, -3)));
        assert_eq!(result.get(36), None);

        Ok(())
    }
}
