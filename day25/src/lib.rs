#![warn(rust_2018_idioms, missing_debug_implementations)]
mod drone;
mod error;
mod interactive;
mod santa;

use computer::Code;
use drone::Drone;
use error::DroneError;
use interactive::Interactive;

pub fn result() -> Result<(), DroneError> {
    let code = Code::from_file("day25", "input.txt")?;

    let mut drone = Drone::new(&code);
    let result = drone.run()?;

    println!("Day 25 - Result 1: {}", result);

    Ok(())
}

pub fn interactive() -> Result<(), DroneError> {
    let code = Code::from_file("day25", "input.txt")?;
    let mut interactive = Interactive::new(&code);

    interactive.run()
}
