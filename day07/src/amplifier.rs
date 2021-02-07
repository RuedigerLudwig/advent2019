use std::collections::VecDeque;

use computer::{Code, ListInput, VirtualMachine};

use crate::{error::AmplifierError, permutations::Permutate};

pub struct Amplifier<'a> {
    computers: VecDeque<(ListInput, VirtualMachine<'a>)>,
}

impl Amplifier<'_> {
    pub fn get_best(template: Code, setting: &Vec<i64>) -> Result<i64, AmplifierError> {
        let mut result = i64::MIN;
        for perm in setting.permutate() {
            let mut amplifier = Amplifier::new(template.clone(), &perm);
            result = result.max(amplifier.run_once(0)?);
        }
        Ok(result)
    }

    pub fn get_best_continously(template: Code, setting: &Vec<i64>) -> Result<i64, AmplifierError> {
        let mut result = i64::MIN;
        for perm in setting.permutate() {
            let mut amplifier = Amplifier::new(template.clone(), &perm);
            result = result.max(amplifier.run_continously(0)?);
        }
        Ok(result)
    }

    fn new<'a>(code: Code, setting: &[&i64]) -> Amplifier<'a> {
        let computers = setting
            .iter()
            .map(|&value| {
                let mut input = ListInput::new();
                input.provide_input(*value);
                let vm = VirtualMachine::new(code.clone(), input.clone());
                (input, vm)
            })
            .collect();
        Amplifier { computers }
    }

    fn run_once(&mut self, initial_value: i64) -> Result<i64, AmplifierError> {
        let mut value = initial_value;
        for (input, vm) in self.computers.iter_mut() {
            input.provide_input(value);
            let result = vm.get_all()?;
            if result.len() != 1 {
                return Err(AmplifierError::NotExactlyOne);
            }
            value = result[0];
        }
        Ok(value)
    }

    fn run_continously(&mut self, initial_value: i64) -> Result<i64, AmplifierError> {
        let mut end_value = initial_value;
        loop {
            if let Some((mut input, mut vm)) = self.computers.pop_front() {
                input.provide_input(end_value);
                if let Some(step_result) = vm.next()? {
                    end_value = step_result;
                    self.computers.push_back((input, vm));
                } else {
                    break;
                }
            }
        }
        Ok(end_value)
    }
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

        let mut amplifier = Amplifier::new(code, &vec![&4, &3, &2, &1, &0]);
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
        let result = Amplifier::get_best(code, &vec![0, 1, 2, 3, 4])?;

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
        let result = Amplifier::get_best(code, &vec![0, 1, 2, 3, 4])?;

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

        let mut amplifier = Amplifier::new(code, &vec![&9, &8, &7, &6, &5]);
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

        let result = Amplifier::get_best_continously(code, &vec![5, 6, 7, 8, 9])?;

        assert_eq!(result, expected);

        Ok(())
    }
}
