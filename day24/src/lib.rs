mod eris;
mod eris_recursive;

use common::read_all_lines;
use eris::Eris;
use eris_recursive::ErisRecursive;
use std::error::Error;

pub fn result() -> Result<(), Box<dyn Error>> {
    let input = read_all_lines("day24", "input.txt")?;

    let eris = Eris::parse(&input);
    let result1 = eris.run_till_stable().rate();
    println!("Day 24 - Result 1: {}", result1);

    let eris2 = ErisRecursive::parse(&input);
    let result2 = eris2.repeat(200).count_ants();
    println!("Day 24 - Result 2: {}", result2);

    Ok(())
}
