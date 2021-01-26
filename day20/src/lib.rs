#![warn(rust_2018_idioms)]

mod explorer;
mod explorer_two;
mod map;
mod map_error;
mod paths;

use std::error::Error;

use common::read_all_lines;
use explorer::Explorer;
use explorer_two::ExplorerTwo;
use map::Map;

pub fn result() -> Result<(), Box<dyn Error>> {
    let input = read_all_lines("day20", "input.txt")?;
    let map = Map::parse(&input)?;
    let result1 = Explorer::new(&map).explore()?;
    println!("Day 20 - Result 1: {}", result1);

    let result2 = ExplorerTwo::new(&map).explore()?;
    println!("Day 20 - Result 2: {}", result2);

    Ok(())
}
