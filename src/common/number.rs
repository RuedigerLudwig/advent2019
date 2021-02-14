use std::ops::{Add, AddAssign, BitAnd, Div, Mul, Rem, Shl, Shr, Sub, SubAssign};

pub trait Number:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Shr<Output = Self>
    + Shl<Output = Self>
    + AddAssign
    + SubAssign
    + BitAnd<Output = Self>
    + PartialEq
    + PartialOrd
    + Ord
{
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;

    fn abs(self) -> Self;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn rem_euclid(self, rhs: Self) -> Self;

    fn is_odd(&self) -> bool {
        *self & Self::ONE == Self::ONE
    }

    fn is_even(&self) -> bool {
        !self.is_odd()
    }

    fn as_f64(self) -> f64;
}

impl Number for i32 {
    const ZERO: i32 = 0i32;
    const ONE: i32 = 1i32;
    const MAX: i32 = i32::MAX;

    fn abs(self) -> Self {
        (self as i32).abs()
    }

    fn rem_euclid(self, rhs: Self) -> Self {
        (self as i32).rem_euclid(rhs)
    }

    fn checked_mul(self, rhs: Self) -> Option<Self> {
        (self as i32).checked_mul(rhs)
    }

    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl Number for i64 {
    const ZERO: i64 = 0i64;
    const ONE: i64 = 1i64;
    const MAX: i64 = i64::MAX;

    fn abs(self) -> Self {
        (self as i64).abs()
    }

    fn rem_euclid(self, rhs: Self) -> Self {
        (self as i64).rem_euclid(rhs)
    }

    fn checked_mul(self, rhs: Self) -> Option<Self> {
        (self as i64).checked_mul(rhs)
    }

    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl Number for i128 {
    const ZERO: i128 = 0i128;
    const ONE: i128 = 1i128;
    const MAX: i128 = i128::MAX;

    fn abs(self) -> Self {
        (self as i128).abs()
    }

    fn rem_euclid(self, rhs: Self) -> Self {
        (self as i128).rem_euclid(rhs)
    }

    fn checked_mul(self, rhs: Self) -> Option<Self> {
        (self as i128).checked_mul(rhs)
    }

    fn as_f64(self) -> f64 {
        self as f64
    }
}
