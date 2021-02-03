#![warn(rust_2018_idioms, missing_debug_implementations)]
use common::read_all_lines;
use error::JupiterError;
use jupiter::Jupiter;

mod error;
mod jupiter;

pub fn result() -> Result<(), JupiterError> {
    let input = read_all_lines("day12", "input.txt")?;
    let system = Jupiter::parse(&input)?;
    let result = system.step(1000);

    println!("Day 12 - Result 1: {}", result.get_energy());

    println!("Day 12 - Result 2: {}", system.get_repeat_steps());

    Ok(())
}
