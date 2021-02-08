mod better_password;
mod error;
mod password;

use crate::common::file::read_data;
use better_password::check_better;
use error::PasswordError;
use password::check;

pub fn result() -> Result<(), PasswordError> {
    let input = read_data("day04", "input.txt")?;
    let split: Vec<i32> = input
        .split('-')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    if split.len() != 2 || split[0] >= split[1] {
        return Err(PasswordError::RangeError);
    }

    let result1 = check(split[0], split[1]);
    println!("Day 04 - Result 1: {}", result1);

    let result2 = check_better(split[0], split[1]);
    println!("Day 04 - Result 2: {}", result2);

    Ok(())
}
