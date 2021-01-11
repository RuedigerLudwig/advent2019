use common::{read_single_line, CommonError};
use fft::Transmission;

mod fft;

pub fn result() -> Result<(), CommonError> {
    let input = read_single_line("day16", "input.txt")?;
    let fft = Transmission::parse(&input)?;

    let result1 = fft.run_small();
    println!("Day 16 - Result 1: {}", result1);

    let result2 = fft.run_big();
    println!("Day 16 - Result 2: {}", result2);

    Ok(())
}
