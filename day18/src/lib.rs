use common::read_all_lines;
use map::Map;
use std::error::Error;
use vault::{find_all_keys_part1, find_all_keys_part2};

mod content;
mod explorer;
mod map;
mod path;
mod vault;
mod vault_error;

pub fn result() -> Result<(), Box<dyn Error>> {
    let input = read_all_lines("day18", "input.txt")?;
    let map = Map::new(&input)?;
    let result1 = find_all_keys_part1(&map)?;
    println!("Day 18 - Result 1: {}", result1);

    let result2 = find_all_keys_part2(&map)?;
    println!("Day 18 - Result 2: {}", result2);

    Ok(())
}
