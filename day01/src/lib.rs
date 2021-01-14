use common::{as_int, read_all_lines, CommonError};

fn calculate(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calculate2(mass: i32) -> i32 {
    let mut result = 0;
    let mut fuel = calculate(mass);
    while fuel > 0 {
        result += fuel;
        fuel = calculate(fuel);
    }
    result
}

fn do_iter<F>(numbers: &[i32], function: F) -> i32
where
    F: Fn(i32) -> i32,
{
    numbers.iter().copied().map(function).sum()
}

pub fn result() -> Result<(), CommonError> {
    let numbers = read_all_lines("day01", "input.txt")?
        .iter()
        .map(|s| as_int(s))
        .collect::<Result<Vec<_>, _>>()?;

    let result1 = do_iter(&numbers, calculate);
    println!("Day 01 - Result 1: {}", result1);

    let result2 = do_iter(&numbers, calculate2);
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
