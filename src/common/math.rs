use thiserror::Error;

use super::number::Number;

#[derive(Error, Debug)]
pub enum MathError {
    #[error("We can not calculate so close to the ceiling")]
    TooHigh,

    #[error("Need positive modulo")]
    NeedPositiveModulo,

    #[error("Need non negative exponent")]
    NeedNonNegativeExponent,
}

fn non_zero_gcd<T>(mut a: T, mut b: T) -> T
where
    T: Number,
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
    T: Number,
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
    T: Number,
{
    if a == T::ZERO || b == T::ZERO {
        T::ZERO
    } else {
        a * b / non_zero_gcd(a, b)
    }
}

pub fn modulus_inv<T>(num: T, modulo: T) -> Option<T>
where
    T: Number,
{
    let num = num.rem_euclid(modulo);
    let mut s = (T::ZERO, T::ONE);
    let mut r = (modulo, num);
    while r.0 != T::ZERO {
        let q = r.1 / r.0;
        r = (r.1 - q * r.0, r.0);
        s = (s.1 - q * s.0, s.0);
    }
    if r.1 != T::ONE {
        None
    } else {
        Some(s.1.rem_euclid(modulo))
    }
}

pub fn modulus_mul<T>(a: T, b: T, modulo: T) -> Result<T, MathError>
where
    T: Number,
{
    let mul = if let Some(mul) = a.checked_mul(b) {
        mul
    } else if T::MAX >> T::ONE >= a {
        let start = if b.is_odd() { a } else { T::ZERO };
        start + modulus_mul((a << T::ONE).rem_euclid(modulo), b >> T::ONE, modulo)?
    } else {
        return Err(MathError::TooHigh);
    };

    Ok(mul.rem_euclid(modulo))
}

pub fn modulus_exp<T>(base: T, exponent: T, modulo: T) -> Result<T, MathError>
where
    T: Number,
{
    if modulo < T::ONE {
        return Err(MathError::NeedPositiveModulo);
    }
    if exponent < T::ZERO {
        return Err(MathError::NeedNonNegativeExponent);
    }

    if modulo == T::ONE {
        Ok(T::ZERO)
    } else {
        let mut result = T::ONE;
        let mut base = base.rem_euclid(modulo);
        let mut exponent = exponent;
        while exponent > T::ZERO {
            if exponent.is_odd() {
                result = modulus_mul(result, base, modulo)?;
            }
            exponent = exponent >> T::ONE;
            base = modulus_mul(base, base, modulo)?;
        }
        Ok(result)
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

    #[test]
    fn test_modulo_mul() -> Result<(), MathError> {
        let a = 1_234_567_890_123_456i64;
        let b = 98_765;
        let result = modulus_mul(a, b, 3_333_333_333_333_333)?;

        assert_eq!(result, 2_097_668_043_144_033);

        Ok(())
    }

    #[test]
    fn test_modulo_exp() -> Result<(), MathError> {
        let base = 4;
        let exponent = 13;
        let modulo = 497;
        let result = modulus_exp(base, exponent, modulo)?;

        assert_eq!(result, 445);

        Ok(())
    }

    #[test]
    fn test_inverse_modulo() {
        let num = 3;
        let modulo = 10;
        let inv = modulus_inv(num, modulo);

        assert_eq!(inv, Some(7));
    }
}
