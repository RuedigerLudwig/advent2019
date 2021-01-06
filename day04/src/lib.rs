use better_password::check_better;
use common::{as_int, read_single_line, CommonError};
use password::check;
mod better_password;
mod password;

pub fn result() -> Result<(), CommonError> {
    let input = read_single_line("day04", "input.txt")?;
    let split: Vec<i32> = input.split('-').map(as_int).collect::<Result<_, _>>()?;
    if split.len() != 2 || split[0] >= split[1] {
        return Err(CommonError::MessageError(String::from("Need range of two")));
    }

    let result1 = check(split[0], split[1]);
    println!("Day 04 - Result 1: {}", result1);

    let result2 = check_better(split[0], split[1]);
    println!("Day 04 - Result 2: {}", result2);

    Ok(())
}
