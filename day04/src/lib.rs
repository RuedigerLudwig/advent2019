#![warn(rust_2018_idioms, missing_debug_implementations)]
mod better_password;
mod error;
mod password;

use better_password::check_better;
use common::read_single_line;
use error::PasswordError;
use password::check;

pub fn result() -> Result<(), PasswordError> {
    let input = read_single_line("day04", "input.txt")?;
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
