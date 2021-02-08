mod error;
mod fft;

use crate::common::file::read_data;
use error::FftError;
use fft::Transmission;

pub fn result() -> Result<(), FftError> {
    let input = read_data("day16", "input.txt")?;
    let fft: Transmission = input.parse()?;

    let result1 = fft.run_small();
    println!("Day 16 - Result 1: {}", result1);

    let result2 = fft.run_big();
    println!("Day 16 - Result 2: {}", result2);

    Ok(())
}
