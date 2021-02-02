#![warn(rust_2018_idioms)]
mod card_error;
mod cards;

use std::error::Error;

use cards::{CardShuffle, Techniques};
use common::read_all_lines;

pub fn result() -> Result<(), Box<dyn Error>> {
    let input = Techniques::parse(&read_all_lines("day22", "input.txt")?)?;

    let shuffle = CardShuffle::create(&input, 10_007)?.invert()?;
    let result1 = shuffle.get_position_of_card(2019);
    println!("Day 22 - Result 1: {}", result1);

    let shuffle = CardShuffle::create(&input, 119_315_717_514_047)?.repeat(101_741_582_076_661)?;
    let result2 = shuffle.get_position_of_card(2020);

    println!("Day 22 - Result 2: {}", result2);
    Ok(())
}
