use std::collections::VecDeque;

use computer::{Code, ListInput, VirtualMachine};

use crate::{error::AmplifierError, permutations::LexPermutations};

pub struct Amplifier<'a> {
    computers: VecDeque<(ListInput, VirtualMachine<'a>)>,
}

impl Amplifier<'_> {
    pub fn new<'a>(code: Code, setting: &Vec<i64>) -> Amplifier<'a> {
        let computers = setting
            .iter()
            .map(|value| {
                let mut input = ListInput::new();
                input.provide_input(*value);
                let vm = VirtualMachine::new(code.clone(), input.clone());
                (input, vm)
            })
            .collect();
        Amplifier { computers }
    }

    pub fn get_best(template: Code, setting: &[i64]) -> Result<i64, AmplifierError> {
        let mut result = i64::MIN;
        for perm in LexPermutations::new(setting) {
            let mut amplifier = Amplifier::new(template.clone(), &perm);
            result = result.max(amplifier.run_once(0)?);
        }
        Ok(result)
    }

    pub fn get_best_continously(template: Code, setting: &Vec<i64>) -> Result<i64, AmplifierError> {
        let mut result = i64::MIN;
        for perm in LexPermutations::new(setting) {
            let mut amplifier = Amplifier::new(template.clone(), &perm);
            result = result.max(amplifier.run_continously(0)?);
        }
        Ok(result)
    }

    pub fn run_once(&mut self, initial_value: i64) -> Result<i64, AmplifierError> {
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

    pub fn run_continously(&mut self, initial_value: i64) -> Result<i64, AmplifierError> {
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
