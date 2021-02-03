#![warn(rust_2018_idioms, missing_debug_implementations)]
use common::read_all_lines;
mod error;
mod orbits;

use error::OrbitError;
use orbits::System;

pub fn result() -> Result<(), OrbitError> {
    let input = read_all_lines("day06", "input.txt")?;
    let system = System::parse(&input)?;

    println!("Day 06 - Result 1: {}", system.count_orbits());

    let result2 = system.count_transfers("YOU", "SAN")?;
    println!("Day 06 - Result 2: {}", result2);

    Ok(())
}

