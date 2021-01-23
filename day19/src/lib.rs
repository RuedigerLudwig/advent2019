mod interface;
mod tractor;
mod tractor_error;

use std::error::Error;

use computer::Code;
use interface::TractorComputerInterface;
use tractor::Tractor;

pub fn result() -> Result<(), Box<dyn Error>> {
    let code = Code::from_file("day19", "input.txt")?;
    let interface = TractorComputerInterface::new(&code);
    let mut droid = Tractor::new(interface);
    let result1 = droid.scan(50)?;
    println!("Day 19 - Result 1: {}", result1);

    let result2 = droid.fit(100)?;
    println!("Day 19 - Result 2: {}", result2);

    Ok(())
}
