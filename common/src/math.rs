use std::ops::{Add, Div, Mul, Rem};

pub trait GcdNum: Copy + Add + Mul + Div + Rem + Eq + PartialEq {
    const ZERO: Self;
    fn abs(self) -> Self;
    fn get_rem(self, other: Self) -> Self;
    fn mul_and_div(self, fst: Self, snd: Self) -> Self;
}

impl GcdNum for i32 {
    const ZERO: i32 = 0i32;

    fn abs(self) -> Self {
        (self as i32).abs()
    }

    fn get_rem(self, other: Self) -> Self {
        self % other
    }

    fn mul_and_div(self, fst: Self, snd: Self) -> Self {
        self * fst / snd
    }
}

impl GcdNum for i64 {
    const ZERO: i64 = 0i64;

    fn abs(self) -> Self {
        (self as i64).abs()
    }

    fn get_rem(self, other: Self) -> Self {
        self % other
    }

    fn mul_and_div(self, fst: Self, snd: Self) -> Self {
        self * fst / snd
    }
}

fn non_zero_gcd<T: GcdNum>(a: T, b: T) -> T {
    let mut a = a;
    let mut b = b;
    while b != T::ZERO {
        let t = a.get_rem(b);
        a = b;
        b = t;
    }
    a.abs()
}

pub fn gcd<T: GcdNum>(a: T, b: T) -> T {
    if a == T::ZERO {
        b.abs()
    } else if b == T::ZERO {
        a.abs()
    } else {
        non_zero_gcd(a, b)
    }
}

pub fn lcm<T: GcdNum>(a: T, b: T) -> T {
    if a == T::ZERO || b == T::ZERO {
        T::ZERO
    } else {
        a.mul_and_div(b, non_zero_gcd(a, b))
    }
}
