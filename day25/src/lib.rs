mod drone;
mod drone_error;
mod interactive;
mod santa;

use computer::Code;
use drone::Drone;
use interactive::Interactive;
use std::error::Error;

pub fn result() -> Result<(), Box<dyn Error>> {
    let code = Code::from_file("day25", "input.txt")?;

    let mut drone = Drone::new(&code);
    let result = drone.run()?;

    println!("Day 25 - Result 1: {}", result);

    Ok(())
}

pub fn interactive() -> Result<(), Box<dyn Error>> {
    let code = Code::from_file("day25", "input.txt")?;
    let mut interactive = Interactive::new(&code);

    interactive.run()
}
