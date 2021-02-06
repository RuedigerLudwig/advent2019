#![warn(rust_2018_idioms, missing_debug_implementations)]
mod eris_plain;
mod eris_recursive;

use common::file::read_data;
use eris_plain::Eris;
use eris_recursive::ErisRecursive;

pub fn result() -> Result<(), std::io::Error> {
    let input = read_data("day24", "input.txt")?;

    let eris = Eris::parse(&input);
    let result1 = eris.run_till_stable().rate();
    println!("Day 24 - Result 1: {}", result1);

    let eris2 = ErisRecursive::parse(&input);
    let result2 = eris2.repeat(200).count_bugs();
    println!("Day 24 - Result 2: {}", result2);

    Ok(())
}
