use computer::{computer_error::ComputerError, Computer};
use droid::Droid;
use interface::ComputerInterface;

mod droid;
mod interface;

pub fn result() -> Result<(), ComputerError> {
    let template = Computer::from_file("day15", "input.txt")?;
    let interface = ComputerInterface::new(&template);
    let mut droid = Droid::new(interface);
    let result = droid.explore()?;

    println!("Day 15 - Result 1: {}\n{}", result, droid);

    Ok(())
}
