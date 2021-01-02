pub fn check_password(input: &str) -> bool {
    if input.len() != 6 {
        return false;
    }

    let mut adjacent = false;
    let mut last_seen = '/';

    for digit in input.chars() {
        if !digit.is_ascii_digit() || digit < last_seen {
            return false;
        }
        if digit == last_seen {
            adjacent = true;
        }
        last_seen = digit;
    }

    adjacent
}

pub fn check(from: i32, to: i32) -> i32 {
    let mut result = 0;
    for next in from..(to + 1) {
        let input = next.to_string();
        if check_password(&input) {
            result += 1
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::CommonError;

    #[test]
    fn test_one() -> Result<(), CommonError> {
        let input = "111111";
        let expected = true;
        let result = check_password(&input);
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_two() -> Result<(), CommonError> {
        let input = "223450";
        let expected = false;
        let result = check_password(&input);
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_three() -> Result<(), CommonError> {
        let input = "123789";
        let expected = false;
        let result = check_password(&input);
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_four() -> Result<(), CommonError> {
        let input = "123788";
        let expected = true;
        let result = check_password(&input);
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_more() -> Result<(), CommonError> {
        let expected = 2;
        let result = check(111110, 111112);
        assert_eq!(expected, result);

        Ok(())
    }
}
