use std::{cmp::Ordering, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(i32, i32);

impl Pos {
    pub fn origin() -> Pos {
        Pos(0, 0)
    }

    pub fn new(row: i32, col: i32) -> Pos {
        Pos(row, col)
    }

    pub fn row(&self) -> i32 {
        self.0
    }

    pub fn col(&self) -> i32 {
        self.1
    }

    pub fn abs(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.abs().cmp(&other.abs())
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl std::ops::Add for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<(i32, i32)> for Pos {
    type Output = Self;
    fn add(self, rhs: (i32, i32)) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<i32> for Pos {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
