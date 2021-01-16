use computer::Code;
use droid::Droid;
use interface::ComputerInterface;
use std::error::Error;

mod droid;
mod interface;

pub fn result() -> Result<(), Box<dyn Error>> {
    let code = Code::from_file("day15", "input.txt")?;
    let interface = ComputerInterface::new(&code);
    let mut droid = Droid::new(interface);

    let result1 = droid.explore()?;
    println!("Day 15 - Result 1: {}", result1);

    let result2 = droid.oxygenize()?;
    println!("Day 15 - Result 2: {}", result2);

    Ok(())
}
