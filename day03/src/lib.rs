use common::read_all_lines;
use std::str::FromStr;
use wire::Wire;
use wire_error::WireError;

mod section;
mod wire;
mod wire_error;

pub fn result() -> Result<(), WireError> {
    let input = read_all_lines("day03", "input.txt")?;
    let wire1 = Wire::from_str(&input[0])?;
    let wire2 = Wire::from_str(&input[1])?;

    if let Some(result) = wire1.get_closest_intersection(&wire2) {
        println!("Day 03 - Result 1: {}", result.abs());
    } else {
        return Err(WireError::MessageError(String::from("No crossing found")));
    }

    if let Some(result) = wire1.get_shortest_intersection(&wire2) {
        println!("Day 03 - Result 2: {}", result);
    } else {
        return Err(WireError::MessageError(String::from("No crossing found")));
    }

    Ok(())
}
