#![warn(rust_2018_idioms, missing_debug_implementations)]

mod error;
mod explorer;
mod explorer_two;
mod map;
mod paths;

use common::read_all_lines;
use error::MapError;
use explorer::Explorer;
use explorer_two::ExplorerTwo;
use map::Map;

pub fn result() -> Result<(), MapError> {
    let input = read_all_lines("day20", "input.txt")?;
    let map = Map::parse(&input)?;
    let result1 = Explorer::new(&map).explore()?;
    println!("Day 20 - Result 1: {}", result1);

    let result2 = ExplorerTwo::new(&map).explore()?;
    println!("Day 20 - Result 2: {}", result2);

    Ok(())
}
