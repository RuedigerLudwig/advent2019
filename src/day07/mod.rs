mod amplifier;
mod error;
mod permutations;

use crate::computer::Code;
use amplifier::Amplifier;
use error::AmplifierError;

pub fn result() -> Result<(), AmplifierError> {
    let code = Code::from_file("day07", "input.txt")?;

    let result1 = Amplifier::get_best(code.clone(), &vec![0, 1, 2, 3, 4])?;
    println!("Day 07 - Result 1: {:?}", result1);

    let result2 = Amplifier::get_best_continously(code, &vec![5, 6, 7, 8, 9])?;
    println!("Day 07 - Result 2: {:?}", result2);

    Ok(())
}
