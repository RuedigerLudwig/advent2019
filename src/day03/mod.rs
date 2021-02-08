mod error;
mod section;
mod wire;

use crate::common::file::read_data;
use error::WireError;
use std::str::FromStr;
use wire::Wire;

pub fn result() -> Result<(), WireError> {
    let input = read_data("day03", "input.txt")?;
    let mut input = input.lines();

    if let Some((input1, input2)) = input.next().zip(input.next()) {
        let wire1 = Wire::from_str(input1)?;
        let wire2 = Wire::from_str(input2)?;

        if let Some(result) = wire1.get_closest_intersection(&wire2) {
            println!("Day 03 - Result 1: {}", result.abs());
        } else {
            return Err(WireError::NoCrossing);
        }

        if let Some(result) = wire1.get_shortest_intersection(&wire2) {
            println!("Day 03 - Result 2: {}", result);
        } else {
            return Err(WireError::NoCrossing);
        }
        Ok(())
    } else {
        panic!("Did not get to wires!")
    }
}
