use better_password::check_better;
use common::{as_int, read_single_line, CommonError};
use password::check;
mod better_password;
mod password;

pub fn result1() -> Result<String, CommonError> {
    let input = read_single_line("data/day04/input.txt")?;
    let split: Vec<i32> = input.split('-').map(as_int).collect::<Result<_, _>>()?;
    if split.len() != 2 || split[0] >= split[1] {
        Err(CommonError::MessageError(String::from("Need range of two")))
    } else {
        let result = check(split[0], split[1]);
        Ok(format!("Day 04 - Result 1: {}", result))
    }
}

pub fn result2() -> Result<String, CommonError> {
    let input = read_single_line("data/day04/input.txt")?;
    let split: Vec<i32> = input.split('-').map(as_int).collect::<Result<_, _>>()?;
    if split.len() != 2 || split[0] >= split[1] {
        Err(CommonError::MessageError(String::from("Need range of two")))
    } else {
        let result = check_better(split[0], split[1]);
        Ok(format!("Day 04 - Result 1: {}", result))
    }
}
