use super::error::JupiterError;
use crate::common::math;
use regex::Regex;
use std::{fmt::Debug, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Component {
    pos: i32,
    vel: i32,
}

impl Component {
    fn apply_gravity(&self, other: &Component) -> i32 {
        (other.pos - self.pos).signum()
    }

    fn apply_velocity(&self, added: i32) -> Component {
        Component {
            pos: self.pos + self.vel + added,
            vel: self.vel + added,
        }
    }

    fn calc_repeat(start: Vec<Component>) -> i64 {
        let mut current = start.clone();
        let mut round = 0;
        loop {
            round += 1;
            let next = current
                .iter()
                .map(|component| {
                    let added = current
                        .iter()
                        .map(|other| component.apply_gravity(other))
                        .sum();
                    component.apply_velocity(added)
                })
                .collect::<Vec<_>>();
            if next == start {
                break round;
            }
            current = next;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Moon {
    x: Component,
    y: Component,
    z: Component,
}

impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            x: Component { pos: x, vel: 0 },
            y: Component { pos: y, vel: 0 },
            z: Component { pos: z, vel: 0 },
        }
    }

    fn apply_gravity(&self, other: &Moon) -> (i32, i32, i32) {
        (
            self.x.apply_gravity(&other.x),
            self.y.apply_gravity(&other.y),
            self.z.apply_gravity(&other.z),
        )
    }

    fn apply_velocity(&self, (dx, dy, dz): (i32, i32, i32)) -> Moon {
        Moon {
            x: self.x.apply_velocity(dx),
            y: self.y.apply_velocity(dy),
            z: self.z.apply_velocity(dz),
        }
    }

    pub fn get_energy(&self) -> i32 {
        (self.x.pos.abs() + self.y.pos.abs() + self.z.pos.abs())
            * (self.x.vel.abs() + self.y.vel.abs() + self.z.vel.abs())
    }

    fn get(&self, index: usize) -> Option<Component> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            _ => None,
        }
    }
}

impl FromStr for Moon {
    type Err = JupiterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>")?;
        if let Some(items) = re.captures(s) {
            let x = items["x"].parse()?;
            let y = items["y"].parse()?;
            let z = items["z"].parse()?;
            Ok(Moon::new(x, y, z))
        } else {
            Err(JupiterError::NoValidMoon(s.to_owned()))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Jupiter {
    moons: Vec<Moon>,
}

impl Jupiter {
    pub fn parse(input: &str) -> Result<Jupiter, JupiterError> {
        Ok(Jupiter {
            moons: input
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        })
    }

    pub fn step(&self, steps: i32) -> Jupiter {
        let mut moons = self.moons.clone();
        for _ in 0..steps {
            let mut next_moons = moons.clone();
            for moon in next_moons.iter_mut() {
                let mut added = (0, 0, 0);
                for other in &moons {
                    let delta = moon.apply_gravity(&other);
                    added = (added.0 + delta.0, added.1 + delta.1, added.2 + delta.2);
                }
                *moon = moon.apply_velocity(added);
            }
            moons = next_moons;
        }

        return Jupiter { moons };
    }

    pub fn get_energy(&self) -> i32 {
        self.moons.iter().map(|moon| moon.get_energy()).sum()
    }

    pub fn get_repeat_steps(&self) -> i64 {
        (0..3)
            .map(|component| {
                Component::calc_repeat(
                    self.moons
                        .iter()
                        .map(|moon| {
                            moon.get(component)
                                .expect("We made sure to only request valid components")
                        })
                        .collect(),
                )
            })
            .fold(1, |a, b| math::lcm(a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::file::read_data;

    pub fn full((xp, xv): (i32, i32), (yp, yv): (i32, i32), (zp, zv): (i32, i32)) -> Moon {
        Moon {
            x: Component { pos: xp, vel: xv },
            y: Component { pos: yp, vel: yv },
            z: Component { pos: zp, vel: zv },
        }
    }

    #[test]
    fn test_parse_moon() -> Result<(), JupiterError> {
        let input = "<x=-1, y=0, z=2>";
        let moon = Moon::from_str(&input)?;
        let expected = Moon::new(-1, 0, 2);

        assert_eq!(moon, expected);

        Ok(())
    }

    #[test]
    fn test_parse_moon0() -> Result<(), JupiterError> {
        let input = read_data("day12", "example1.txt")?;
        let result = Jupiter::parse(&input)?;
        let expected = Jupiter {
            moons: vec![
                Moon::new(-1, 0, 2),
                Moon::new(2, -10, -7),
                Moon::new(4, -8, 8),
                Moon::new(3, 5, -1),
            ],
        };
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_parse_moon1() -> Result<(), JupiterError> {
        let input = read_data("day12", "example1.txt")?;
        let system = Jupiter::parse(&input)?;
        let result = system.step(1);
        let expected = Jupiter {
            moons: vec![
                full((2, 3), (-1, -1), (1, -1)),
                full((3, 1), (-7, 3), (-4, 3)),
                full((1, -3), (-7, 1), (5, -3)),
                full((2, -1), (2, -3), (0, 1)),
            ],
        };
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_parse_moon2() -> Result<(), JupiterError> {
        let input = read_data("day12", "example1.txt")?;
        let system = Jupiter::parse(&input)?;
        let result = system.step(10);
        let expected = 179;
        assert_eq!(result.get_energy(), expected);

        Ok(())
    }

    #[test]
    fn test_get_moon10_2() -> Result<(), JupiterError> {
        let input = read_data("day12", "example2.txt")?;
        let system = Jupiter::parse(&input)?;
        let result = system.step(10);
        let expected = Jupiter {
            moons: vec![
                full((-9, -2), (-10, -2), (1, -1)),
                full((4, -3), (10, 7), (9, -2)),
                full((8, 5), (-10, -1), (-3, -2)),
                full((5, 0), (-10, -4), (3, 5)),
            ],
        };
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_get_moon100_2() -> Result<(), JupiterError> {
        let input = read_data("day12", "example2.txt")?;
        let system = Jupiter::parse(&input)?;
        let result = system.step(100);
        let expected = 1940;
        assert_eq!(result.get_energy(), expected);

        Ok(())
    }

    #[test]
    fn test_repeat_time() -> Result<(), JupiterError> {
        let input = read_data("day12", "example1.txt")?;
        let system = Jupiter::parse(&input)?;
        let result = system.get_repeat_steps();
        let expected = 2772;
        assert_eq!(result, expected);

        Ok(())
    }
}
