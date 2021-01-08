pub mod i32 {
    pub fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            b.abs()
        } else if b == 0 {
            a.abs()
        } else {
            let mut a = a;
            let mut b = b;
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a.abs()
        }
    }

    pub fn lcm(a: i32, b: i32) -> i32 {
        if a == 0 || b == 0 {
            0
        } else {
            let gcd = gcd(a, b);
            a * b / gcd
        }
    }
}

pub mod i64 {
    pub fn gcd(a: i64, b: i64) -> i64 {
        if a == 0 {
            b.abs()
        } else if b == 0 {
            a.abs()
        } else {
            let mut a = a;
            let mut b = b;
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a.abs()
        }
    }

    pub fn lcm(a: i64, b: i64) -> i64 {
        if a == 0 || b == 0 {
            0
        } else {
            let gcd = gcd(a, b);
            a * b / gcd
        }
    }
}
