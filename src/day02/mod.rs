mod error;
mod simple;

use crate::computer::Code;
use error::SimpleError;
use simple::{find_numbers, run_patched};

pub fn result() -> Result<(), SimpleError> {
    let code = Code::from_file("day02", "input.txt")?;

    let result1 = run_patched(code.clone())?;
    println!("Day 02 - Result 1: {}", result1);

    let (noun, verb) = find_numbers(code)?;

    println!("Day 02 - Result 2: {}", 100 * noun + verb);

    Ok(())
}
