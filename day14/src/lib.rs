#![warn(rust_2018_idioms, missing_debug_implementations)]
use common::read_all_lines;
use error::FactoryError;
use factory::Factory;

mod error;
mod factory;

pub fn result() -> Result<(), FactoryError> {
    let input = read_all_lines("day14", "input.txt")?;
    let factory = Factory::new(&input)?;

    let result = factory.ore_per_fuel(1)?;
    println!("Day 14 - Result 1: {}", result);

    let result2 = factory.fuel_for_ore(1_000_000_000_000_i64)?;
    println!("Day 14 - Result 2: {}", result2);

    Ok(())
}
