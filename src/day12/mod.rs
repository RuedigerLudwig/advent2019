use crate::common::file::read_data;
use error::JupiterError;
use jupiter::Jupiter;

mod error;
mod jupiter;

pub fn result() -> Result<(), JupiterError> {
    let input = read_data("day12", "input.txt")?;
    let system = Jupiter::parse(&input)?;
    let result = system.step(1000);

    println!("Day 12 - Result 1: {}", result.get_energy());

    println!("Day 12 - Result 2: {}", system.get_repeat_steps());

    Ok(())
}
