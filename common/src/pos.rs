use std::ops::{Add, Mul, Sub};
use std::{cmp::Ordering, fmt};

use crate::{math::gcd, Direction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pos<T>(T, T);

impl Pos<i32> {
    pub const fn new_const(x: i32, y: i32) -> Pos<i32> {
        Pos(x, y)
    }

    pub fn abs(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }

    pub fn normalize(&self) -> (Pos<i32>, i32) {
        if self.0 == 0 && self.1 == 0 {
            (*self, 1)
        } else {
            let ggt = gcd(self.0, self.1);
            (Pos::new(self.0 / ggt, self.1 / ggt), ggt)
        }
    }

    pub fn angle(&self) -> f64 {
        (self.1 as f64).atan2(self.0 as f64)
    }

    pub fn angle2(&self) -> f64 {
        ((-self.0 as f64).atan2(-self.1 as f64) + std::f64::consts::PI)
            .rem_euclid(2.0 * std::f64::consts::PI)
    }
}

impl Pos<f64> {
    pub fn abs(&self) -> f64 {
        self.0.abs() + self.1.abs()
    }

    pub fn angle(&self) -> f64 {
        self.1.atan2(self.0)
    }

    pub fn angle2(&self) -> f64 {
        ((-self.0).atan2(-self.1) + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI)
    }
}

impl<T> Pos<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T) -> Pos<T> {
        Pos(x, y)
    }

    pub fn x(&self) -> T {
        self.0
    }

    pub fn y(&self) -> T {
        self.1
    }
}

impl Ord for Pos<i32> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.abs().cmp(&other.abs())
    }
}

impl PartialOrd for Pos<i32> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> fmt::Display for Pos<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T> Add for Pos<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> Add<(T, T)> for Pos<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: (T, T)) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Direction> for Pos<i32> {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self {
        let dir = rhs.as_pos();
        Self(self.0 + dir.0, self.1 + dir.1)
    }
}

impl<T> Sub for Pos<T>
where
    T: Sub<Output = T>,
{
    type Output = Pos<T>;
    fn sub(self, rhs: Self) -> Pos<T> {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T> Mul<T> for Pos<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
