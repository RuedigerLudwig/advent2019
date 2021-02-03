use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use common::Pos;

use crate::error::AsteroidError;

pub struct Asteroids {
    field: HashSet<Pos<i32>>,
}

impl Asteroids {
    pub fn parse<T: AsRef<str>>(input: &[T]) -> Asteroids {
        let mut field = HashSet::new();
        for (y, line) in (0..).zip(input.iter()) {
            for (x, place) in (0..).zip(line.as_ref().chars()) {
                if place == '#' {
                    field.insert(Pos::new(x, -y));
                }
            }
        }
        Asteroids { field }
    }

    fn get_visible(&self, center: &Pos<i32>) -> HashSet<Pos<i32>> {
        let mut visible_map: HashMap<Pos<i32>, i32> = HashMap::new();
        for asteroid in &self.field {
            if asteroid != center {
                let diff = asteroid - center;
                let (normal, factor) = diff.normalize();

                visible_map
                    .entry(normal)
                    .and_modify(|old_factor| *old_factor = factor.min(*old_factor))
                    .or_insert(factor);
            }
        }

        visible_map
            .iter()
            .map(|(&normal, &factor)| normal * factor + center)
            .collect()
    }

    pub fn get_best_position(&self) -> Result<(Pos<i32>, usize), AsteroidError> {
        let mut best_center = None;
        let mut max = 0;
        let mut count_best = 0;
        for center in &self.field {
            let visible = self.get_visible(center).len();

            if visible > max {
                max = visible;
                count_best = 1;
                best_center = Some(*center)
            } else if visible == max {
                count_best += 1;
            }
        }

        if count_best == 1 {
            Ok((best_center.expect("We know we have a center"), max))
        } else {
            Err(AsteroidError::NoBestCenter)
        }
    }

    fn pos_cmp<'a, 'b>(pos1: &'a Pos<i32>, pos2: &'b Pos<i32>) -> Ordering {
        pos1.angle2()
            .partial_cmp(&pos2.angle2())
            .expect("We hope for the best that we do not find a NaN asteroid somewhere")
    }

    pub fn get_lasered_asteroids(&self, center: Pos<i32>) -> Vec<Pos<i32>> {
        let mut visible_map: HashMap<Pos<i32>, Vec<i32>> = HashMap::new();
        for asteroid in &self.field {
            if *asteroid != center {
                let diff = asteroid - center;
                let (normal, factor) = diff.normalize();

                visible_map
                    .entry(normal)
                    .and_modify(|lst| {
                        lst.push(factor);
                        lst.sort_by(|a, b| b.cmp(a))
                    })
                    .or_insert(vec![factor]);
            }
        }

        let mut sorted_normals = visible_map.keys().copied().collect::<Vec<_>>();
        sorted_normals.sort_by(Asteroids::pos_cmp);

        let mut result = Vec::new();
        loop {
            let mut lasered_any = false;
            for normal in &sorted_normals {
                if let Some(lst) = visible_map.get_mut(normal) {
                    if let Some(factor) = lst.pop() {
                        result.push(center + normal * factor);
                        lasered_any = true;
                    }
                }
            }
            if !lasered_any {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{hashset, read_all_lines};

    #[test]
    fn test_parse_asteroids() -> Result<(), AsteroidError> {
        let input = read_all_lines("day10", "example1.txt")?;
        let asteroids = Asteroids::parse(&input);
        let expected = 10;
        assert_eq!(asteroids.field.len(), expected);

        Ok(())
    }

    #[test]
    fn test_visible_asteroids() -> Result<(), AsteroidError> {
        let input = read_all_lines("day10", "example1.txt")?;
        let asteroids = Asteroids::parse(&input);
        let expected = hashset!(
            Pos::new(1, 0),
            Pos::new(4, 0),
            Pos::new(3, -2),
            Pos::new(4, -3),
            Pos::new(3, -4)
        );
        let result = asteroids.get_visible(&Pos::new(4, -2));
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_best_position() -> Result<(), AsteroidError> {
        let input = read_all_lines("day10", "example1.txt")?;
        let asteroids = Asteroids::parse(&input);
        let expected = (Pos::new(3, -4), 8);
        let result = asteroids.get_best_position()?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_best_position2() -> Result<(), AsteroidError> {
        let input = read_all_lines("day10", "example2.txt")?;
        let asteroids = Asteroids::parse(&input);
        let expected = (Pos::new(5, -8), 33);
        let result = asteroids.get_best_position()?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_laser_order() -> Result<(), AsteroidError> {
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
