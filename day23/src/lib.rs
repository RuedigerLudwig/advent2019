#![warn(rust_2018_idioms, missing_debug_implementations)]
mod error;
mod network;

use computer::Code;
use error::NetworkError;
use network::Switch;

pub fn result() -> Result<(), NetworkError> {
    let code = Code::from_file("day23", "input.txt")?;

    let result1 = Switch::part1(code.clone(), 50)?;
    println!("Day 23 - Result 1: {}", result1);

    let result2 = Switch::part2(code.clone(), 50)?;
    println!("Day 23 - Result 2: {}", result2);
    Ok(())
}
