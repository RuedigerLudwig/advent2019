use super::{direction::Direction, math::gcd, number::Number};
use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pos<T>(T, T)
where
    T: Number;

impl<T> Pos<T>
where
    T: Number,
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

    pub fn max_components(&self, other: &Pos<T>) -> Self {
        Self(self.0.max(other.0), self.1.max(other.1))
    }

    pub fn min_components(&self, other: &Pos<T>) -> Self {
        Self(self.0.min(other.0), self.1.min(other.1))
    }

    pub fn abs(&self) -> T {
        self.0.abs() + self.1.abs()
    }

    pub fn normalize(&self) -> (Pos<T>, T) {
        if self.0 == T::ZERO && self.1 == T::ZERO {
            (*self, T::ONE)
        } else {
            let ggt = gcd(self.0, self.1);
            (Pos::new(self.0 / ggt, self.1 / ggt), ggt)
        }
    }

    pub fn angle(&self) -> f64 {
        self.1.as_f64().atan2(self.0.as_f64())
    }

    pub fn angle2(&self) -> f64 {
        (-self.0.as_f64().atan2(-self.1.as_f64()) + std::f64::consts::PI)
            .rem_euclid(2.0 * std::f64::consts::PI)
    }
}

impl<T> fmt::Display for Pos<T>
where
    T: Number + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T> Add for Pos<T>
where
    T: Number,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> Add for &Pos<T>
where
    T: Number,
{
    type Output = <Pos<T> as Add<Pos<T>>>::Output;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::add(*self, *rhs)
    }
}

impl<T> Add<&Pos<T>> for Pos<T>
where
    T: Number,
{
    type Output = <Pos<T> as Add<Pos<T>>>::Output;
    fn add(self, rhs: &Self) -> Self::Output {
        Pos::add(self, *rhs)
    }
}

impl<T> Add<Pos<T>> for &Pos<T>
where
    T: Number,
{
    type Output = <Pos<T> as Add<Pos<T>>>::Output;
    fn add(self, rhs: Pos<T>) -> Self::Output {
        Pos::add(*self, rhs)
    }
}

impl<T> Add<(T, T)> for Pos<T>
where
    T: Number,
{
    type Output = Self;
    fn add(self, rhs: (T, T)) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Direction> for Pos<i32> {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Pos::add(self, rhs.as_pos())
    }
}

impl Add<&Direction> for Pos<i32> {
    type Output = Self;

    fn add(self, rhs: &Direction) -> Self::Output {
        Pos::add(self, rhs.as_pos())
    }
}

impl Add<&Direction> for &Pos<i32> {
    type Output = Pos<i32>;

    fn add(self, rhs: &Direction) -> Self::Output {
        Pos::add(*self, rhs.as_pos())
    }
}

impl Add<Direction> for &Pos<i32> {
    type Output = Pos<i32>;

    fn add(self, rhs: Direction) -> Self::Output {
        Pos::add(*self, rhs.as_pos())
    }
}

impl<T> Sub for Pos<T>
where
    T: Number,
{
    type Output = Pos<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T> Sub<&Self> for Pos<T>
where
    T: Number,
{
    type Output = Pos<T>;
    fn sub(self, rhs: &Self) -> Self::Output {
        Pos::sub(self, *rhs)
    }
}

impl<T> Sub for &Pos<T>
where
    T: Number,
{
    type Output = Pos<T>;
    fn sub(self, rhs: &Pos<T>) -> Self::Output {
        Pos::sub(*self, *rhs)
    }
}

impl<T> Sub<Pos<T>> for &Pos<T>
where
    T: Number,
{
    type Output = Pos<T>;
    fn sub(self, rhs: Pos<T>) -> Self::Output {
        Pos::sub(*self, rhs)
    }
}

impl<T> Mul<T> for Pos<T>
where
    T: Number,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Pos(self.0 * rhs, self.1 * rhs)
    }
}

impl<T> Mul<T> for &Pos<T>
where
    T: Number,
{
    type Output = Pos<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Pos::mul(*self, rhs)
    }
}

impl<T> Mul<&T> for Pos<T>
where
    T: Number,
{
    type Output = Pos<T>;
    fn mul(self, rhs: &T) -> Self::Output {
        Pos::mul(self, *rhs)
    }
}

impl<T> Mul<&T> for &Pos<T>
where
    T: Number,
{
    type Output = Pos<T>;
    fn mul(self, rhs: &T) -> Self::Output {
        Pos::mul(*self, *rhs)
    }
}
