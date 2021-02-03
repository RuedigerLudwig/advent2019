#![warn(rust_2018_idioms, missing_debug_implementations)]
mod eris_plain;
mod eris_recursive;

use common::{error::CommonError, read_all_lines};
use eris_plain::Eris;
use eris_recursive::ErisRecursive;

pub fn result() -> Result<(), CommonError> {
    let input = read_all_lines("day24", "input.txt")?;

    let eris = Eris::parse(&input);
    let result1 = eris.run_till_stable().rate();
    println!("Day 24 - Result 1: {}", result1);

    let eris2 = ErisRecursive::parse(&input);
    let result2 = eris2.repeat(200).count_bugs();
    println!("Day 24 - Result 2: {}", result2);

    Ok(())
}
