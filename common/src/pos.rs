use std::{cmp::Ordering, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(i32, i32);

impl Pos {
    pub fn origin() -> Pos {
        Pos(0, 0)
    }

    pub fn new(x: i32, y: i32) -> Pos {
        Pos(x, y)
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn abs(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }

    pub fn normalize(&self) -> (Pos, i32) {
        if self.0 == 0 && self.1 == 0 {
            (*self, 1)
        } else {
            let ggt = self.ggt();
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

    fn ggt(&self) -> i32 {
        let mut a = self.0;
        let mut b = self.1;
        if a == 0 {
            b.abs()
        } else {
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a.abs()
        }
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
    type Output = Pos;
    fn sub(self, rhs: Self) -> Pos {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<i32> for Pos {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
