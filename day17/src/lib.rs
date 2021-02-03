#![warn(rust_2018_idioms, missing_debug_implementations)]
mod error;
mod exterior;
mod interface;
mod path;

use computer::Code;
use error::ExteriorError;
use exterior::Exterior;
use interface::ExteriorComputerInterface;

pub fn result() -> Result<(), ExteriorError> {
    let code = Code::from_file("day17", "input.txt")?;
    let interface = ExteriorComputerInterface::new(&code);
    let mut exterior = Exterior::new(interface)?;

    let result1 = exterior.get_alignment();
    println!("Day 17 - Result 1: {}", result1);

    let result2 = exterior.run_bot(true)?;
    println!("Day 17 - Result 2: {}", result2);

    Ok(())
}
