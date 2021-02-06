#![warn(rust_2018_idioms, missing_debug_implementations)]
mod error;
mod orbits;

use common::file::read_data;
use error::OrbitError;
use orbits::System;

pub fn result() -> Result<(), OrbitError> {
    let input = read_data("day06", "input.txt")?;
    let system = System::parse(&input)?;

    println!("Day 06 - Result 1: {}", system.count_orbits());

    let result2 = system.count_transfers("YOU", "SAN")?;
    println!("Day 06 - Result 2: {}", result2);

    Ok(())
}
