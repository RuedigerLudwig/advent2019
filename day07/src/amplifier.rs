use std::collections::VecDeque;

use computer::{Code, ListInput, STVirtualMachine};

use crate::{error::AmplifierError, permutations::LexPermutations};

pub struct Amplifier<'a> {
    computers: VecDeque<STVirtualMachine<'a>>,
}

impl Amplifier<'_> {
    pub fn new<'a>(code: &'a Code, setting: &Vec<i64>) -> Amplifier<'a> {
        let computers = setting
            .iter()
            .map(|value| {
                let vm = STVirtualMachine::new(&code, ListInput::new());
                vm.provide_input(*value);
                vm
            })
            .collect();
        Amplifier { computers }
    }

    pub fn get_best(template: &Code, setting: &[i64]) -> Result<i64, AmplifierError> {
        let mut result = i64::MIN;
        for perm in LexPermutations::new(setting) {
            let mut amplifier = Amplifier::new(template, &perm);
            result = result.max(amplifier.run_once(0)?);
        }
        Ok(result)
    }

    pub fn get_best_continously(
        template: &Code,
        setting: &Vec<i64>,
    ) -> Result<i64, AmplifierError> {
        let mut result = i64::MIN;
        for perm in LexPermutations::new(setting) {
            let mut amplifier = Amplifier::new(template, &perm);
            result = result.max(amplifier.run_continously(0)?);
        }
        Ok(result)
    }

    pub fn run_once(&mut self, initial_value: i64) -> Result<i64, AmplifierError> {
        let mut value = initial_value;
        for vm in self.computers.iter_mut() {
            vm.provide_input(value);
            let result = vm.get_output().get_all()?;
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
            if let Some(computer) = self.computers.pop_front() {
                computer.provide_input(end_value);
                if let Some(step_result) = computer.get_output().next()? {
                    end_value = step_result;
                    self.computers.push_back(computer);
                } else {
                    break;
                }
            }
        }
        Ok(end_value)
    }
}
