#![warn(rust_2018_idioms, missing_debug_implementations)]
mod amplifier;
mod error;
mod permutations;

use amplifier::Amplifier;
use computer::Code;
use error::AmplifierError;

pub fn result() -> Result<(), AmplifierError> {
    let code = Code::from_file("day07", "input.txt")?;

    let result1 = Amplifier::get_best(&code, &vec![0, 1, 2, 3, 4])?;
    println!("Day 07 - Result 1: {:?}", result1);

    let result2 = Amplifier::get_best_continously(&code, &vec![5, 6, 7, 8, 9])?;
    println!("Day 07 - Result 2: {:?}", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_outcome() -> Result<(), AmplifierError> {
        let input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let code = input.into();
        let expected = 43210;

        let mut amplifier = Amplifier::new(&code, &vec![4, 3, 2, 1, 0]);
        let result = amplifier.run_once(0)?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn find_best_outcome_once() -> Result<(), AmplifierError> {
        let input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let code = input.into();

        let expected = 43210;
        let result = Amplifier::get_best(&code, &vec![0, 1, 2, 3, 4])?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn find_best_outcome_once2() -> Result<(), AmplifierError> {
        let input = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let code = input.into();

        let expected = 65210;
        let result = Amplifier::get_best(&code, &vec![0, 1, 2, 3, 4])?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn expected_outcome_continously() -> Result<(), AmplifierError> {
        let input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let code = input.into();
        let expected = 139629729;

        let mut amplifier = Amplifier::new(&code, &vec![9, 8, 7, 6, 5]);
        let result = amplifier.run_continously(0)?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn find_best_outcome_continously1() -> Result<(), AmplifierError> {
        let input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let code = input.into();
        let expected = 139629729;

        let result = Amplifier::get_best_continously(&code, &vec![5, 6, 7, 8, 9])?;

        assert_eq!(result, expected);

        Ok(())
    }
}
