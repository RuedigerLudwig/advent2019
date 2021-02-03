#![warn(rust_2018_idioms, missing_debug_implementations)]
mod droid;
mod error;
mod interface;

use computer::Code;
use droid::Droid;
use error::DroidError;
use interface::ComputerInterface;

pub fn result() -> Result<(), DroidError> {
    let code = Code::from_file("day15", "input.txt")?;
    let interface = ComputerInterface::new(&code);
    let mut droid = Droid::new(interface);

    let result1 = droid.explore()?;
    println!("Day 15 - Result 1: {}", result1);

    let result2 = droid.oxygenize()?;
    println!("Day 15 - Result 2: {}", result2);

    Ok(())
}
