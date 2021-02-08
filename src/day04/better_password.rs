pub fn check_better_password(input: &str) -> bool {
    if input.len() != 6 {
        return false;
    }

    let mut adjacent = false;
    let mut last_seen = '/';
    let mut streak = 0;

    for digit in input.chars() {
        if !digit.is_ascii_digit() || digit < last_seen {
            return false;
        }
        if digit != last_seen {
            if streak == 2 {
                adjacent = true;
            }
            streak = 1;
        } else {
            streak += 1;
        }
        last_seen = digit;
    }

    adjacent || streak == 2
}

pub fn check_better(from: i32, to: i32) -> i32 {
    let mut result = 0;
    for next in from..(to + 1) {
        let input = next.to_string();
        if check_better_password(&input) {
            result += 1
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day04::error::PasswordError;

    #[test]
    fn test_one() -> Result<(), PasswordError> {
        let input = "112233";
        let expected = true;
        let result = check_better_password(&input);
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_two() -> Result<(), PasswordError> {
        let input = "123444";
        let expected = false;
        let result = check_better_password(&input);
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_three() -> Result<(), PasswordError> {
        let input = "111122";
        let expected = true;
        let result = check_better_password(&input);
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_more() -> Result<(), PasswordError> {
        let expected = 1;
        let result = check_better(111121, 111123);
        assert_eq!(expected, result);

        Ok(())
    }
}
