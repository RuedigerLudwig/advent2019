#![warn(rust_2018_idioms, missing_debug_implementations)]
use common::file::read_data;
use error::VaultError;
use map::Map;
use vault::{find_all_keys_part1, find_all_keys_part2};

mod content;
mod error;
mod explorer;
mod map;
mod path;
mod vault;

pub fn result() -> Result<(), VaultError> {
    let input = read_data("day18", "input.txt")?;
    let map = Map::new(&input)?;
    let result1 = find_all_keys_part1(&map)?;
    println!("Day 18 - Result 1: {}", result1);

    let result2 = find_all_keys_part2(&map)?;
    println!("Day 18 - Result 2: {}", result2);

    Ok(())
}
