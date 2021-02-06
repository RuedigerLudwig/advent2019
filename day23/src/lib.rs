#![warn(rust_2018_idioms, missing_debug_implementations)]
mod error;
mod network;

use computer::Code;
use error::NetworkError;
use network::Switch;

pub fn result() -> Result<(), NetworkError> {
    let code = Code::from_file("day23", "input.txt")?;

    let mut switch1 = Switch::new(code.clone(), 50);
    let result1 = switch1.part1()?;
    println!("Day 23 - Result 1: {}", result1);

    let mut switch2 = Switch::new(code.clone(), 50);
    let result2 = switch2.part2()?;
    println!("Day 23 - Result 2: {}", result2);
    Ok(())
}
