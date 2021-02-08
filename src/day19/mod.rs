mod error;
mod interface;
mod tractor;

use crate::computer::Code;
use error::TractorError;
use interface::TractorComputerInterface;
use tractor::Tractor;

pub fn result() -> Result<(), TractorError> {
    let code = Code::from_file("day19", "input.txt")?;
    let interface = TractorComputerInterface::new(code);
    let mut droid = Tractor::new(interface);
    let result1 = droid.scan(50)?;
    println!("Day 19 - Result 1: {}", result1);

    let result2 = droid.fit(100)?;
    println!("Day 19 - Result 2: {}", result2);

    Ok(())
}
