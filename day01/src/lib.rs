use common::{as_int, work_on_file, CommonError};

pub fn calculate(mass: i32) -> i32 {
    mass / 3 - 2
}

pub fn calculate2(mass: i32) -> i32 {
    let mut result = 0;
    let mut fuel = calculate(mass);
    while fuel > 0 {
        result += fuel;
        fuel = calculate(fuel);
    }
    result
}

pub fn result() -> Result<(), CommonError> {
    let result1: i32 = work_on_file("day01", "input.txt", as_int)?
        .iter()
        .copied()
        .map(calculate)
        .sum();

    println!("Day 01 - Result 1: {}", result1);

    let result2: i32 = work_on_file("day01", "input.txt", as_int)?
        .iter()
        .copied()
        .map(calculate2)
        .sum();

    println!("Day 01 - Result 2: {}", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test_1() {
        let result = calculate(12);
        let expected = 2;
        assert_eq!(expected, result);
    }

    #[test]
    fn simple_test_2() {
        let result = calculate(100756);
        let expected = 33583;
        assert_eq!(expected, result);
    }

    #[test]
    fn extended_test_1() {
        let result = calculate2(14);
        let expected = 2;
        assert_eq!(expected, result);
    }

    #[test]
    fn extended_test_2() {
        let result = calculate2(100756);
        let expected = 50346;
        assert_eq!(expected, result);
    }
}
