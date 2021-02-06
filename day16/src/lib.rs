#![warn(rust_2018_idioms, missing_debug_implementations)]
mod error;
mod fft;

use common::file::read_data;
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
