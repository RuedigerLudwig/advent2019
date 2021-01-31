mod network;
mod network_error;

use computer::Code;
use network::Switch;
use std::error::Error;

pub fn result() -> Result<(), Box<dyn Error>> {
    let code = Code::from_file("day23", "input.txt")?;

    let mut switch1 = Switch::new(&code, 50);
    let result1 = switch1.part1()?;
    println!("Day 23 - Result 1: {}", result1);

    let mut switch2 = Switch::new(&code, 50);
    let result2 = switch2.part2()?;
    println!("Day 23 - Result 2: {}", result2);

    Ok(())
}
