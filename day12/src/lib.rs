use common::read_all_lines;
use jupiter::Jupiter;
use jupiter_error::JupiterError;

mod jupiter;
mod jupiter_error;

pub fn result() -> Result<(), JupiterError> {
    let input = read_all_lines("day12", "input.txt")?;
    let system = Jupiter::parse(&input)?;
    let result = system.step(1000);

    println!("Day 12 - Result 1: {}", result.get_energy());

    println!("Day 12 - Result 2: {}", system.get_repeat_steps());

    Ok(())
}
