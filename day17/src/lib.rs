use computer::Code;
use exterior::Exterior;
use interface::ExteriorComputerInterface;
use std::error::Error;

mod exterior;
mod exterior_error;
mod interface;
mod path;

pub fn result() -> Result<(), Box<dyn Error>> {
    let code = Code::from_file("day17", "input.txt")?;
    let interface = ExteriorComputerInterface::new(&code);
    let mut exterior = Exterior::new(interface)?;

    let result1 = exterior.get_alignment();
    println!("Day 17 - Result 1: {}", result1);

    let result2 = exterior.run_bot(true)?;
    println!("Day 17 - Result 2: {}", result2);

    Ok(())
}
