mod cards;
mod error;

use crate::common::file::read_data;
use cards::{CardShuffle, Techniques};
use error::CardError;

pub fn result() -> Result<(), CardError> {
    let input = read_data("day22", "input.txt")?;
    let techniques = Techniques::parse(&input)?;

    let shuffle = CardShuffle::create(&techniques, 10_007)?.invert()?;
    let result1 = shuffle.get_position_of_card(2019)?;
    println!("Day 22 - Result 1: {}", result1);

    let shuffle =
        CardShuffle::create(&techniques, 119_315_717_514_047)?.repeat(101_741_582_076_661)?;
    let result2 = shuffle.get_position_of_card(2020)?;

    println!("Day 22 - Result 2: {}", result2);
    Ok(())
}
