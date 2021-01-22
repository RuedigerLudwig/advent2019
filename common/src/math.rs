use std::ops::{Add, Div, Mul, Rem};

pub trait GcdNum:
    Copy + Add<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Rem<Output = Self> + PartialEq
{
    const ZERO: Self;
    fn abs(self) -> Self;
}

impl GcdNum for i32 {
    const ZERO: i32 = 0i32;

    fn abs(self) -> Self {
        (self as i32).abs()
    }
}

impl GcdNum for i64 {
    const ZERO: i64 = 0i64;

    fn abs(self) -> Self {
        (self as i64).abs()
    }
}

fn non_zero_gcd<T>(mut a: T, mut b: T) -> T
where
    T: GcdNum,
{
    while b != T::ZERO {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: GcdNum,
{
    if a == T::ZERO {
        b.abs()
    } else if b == T::ZERO {
        a.abs()
    } else {
        non_zero_gcd(a, b)
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: GcdNum,
{
    if a == T::ZERO || b == T::ZERO {
        T::ZERO
    } else {
        a * b / non_zero_gcd(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_simple_gcd() {
        assert_eq!(5, gcd(10, 15));
        assert_eq!(7, gcd(21, 49));
        assert_eq!(1, gcd(13, 17));
    }

    #[test]
    fn some_simple_lcm() {
        assert_eq!(18, lcm(6, 9));
        assert_eq!(20, lcm(5, 4));
    }
}
