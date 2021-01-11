use computer::{computer_error::ComputerError, Computer};
use droid::Droid;
use interface::ComputerInterface;

mod droid;
mod interface;

pub fn result() -> Result<(), ComputerError> {
    let template = Computer::from_file("day15", "input.txt")?;
    let interface = ComputerInterface::new(&template);
    let mut droid = Droid::new(interface);

    let result1 = droid.explore()?;
    println!("Day 15 - Result 1: {}", result1);

    let result2 = droid.oxygenize()?;
    println!("Day 15 - Result 2: {}", result2);

    Ok(())
}
