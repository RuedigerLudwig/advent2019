use std::collections::VecDeque;

use computer::{computer_error::ComputerError, Computer};

use crate::permutations::Permutations;

pub struct Amplifier {
    computers: VecDeque<Computer>,
}

impl Amplifier {
    pub fn new(template: &Computer, setting: &Vec<i32>) -> Amplifier {
        let computers = setting
            .iter()
            .map(|input| {
                let mut computer = template.clone();
                computer.provide_input(*input);
                computer
            })
            .collect();
        Amplifier { computers }
    }

    pub fn get_best(template: &Computer, setting: &Vec<i32>) -> Result<i32, ComputerError> {
        let mut result = i32::MIN;
        for perm in Permutations::new(setting) {
            let mut amplifier = Amplifier::new(template, &perm);
            result = result.max(amplifier.run_once(0)?);
        }
        Ok(result)
    }

    pub fn get_best_continously(
        template: &Computer,
        setting: &Vec<i32>,
    ) -> Result<i32, ComputerError> {
        let mut result = i32::MIN;
        for perm in Permutations::new(setting) {
            let mut amplifier = Amplifier::new(template, &perm);
            result = result.max(amplifier.run_continously(0)?);
        }
        Ok(result)
    }

    pub fn run_once(&mut self, initial_value: i32) -> Result<i32, ComputerError> {
        let mut value = initial_value;
        for computer in self.computers.iter_mut() {
            computer.provide_input(value);
            let result = computer.run()?;
            if result.len() != 1 {
                Err(ComputerError::MessageError(format!(
                    "Did not get exactly one result: {:?}",
                    result,
                )))?;
            }
            value = result[0];
        }
        Ok(value)
    }

    pub fn run_continously(&mut self, initial_value: i32) -> Result<i32, ComputerError> {
        let mut value = initial_value;
        loop {
            if let Some(mut computer) = self.computers.pop_front() {
                computer.provide_input(value);
                if let Some(result) = computer.next() {
                    value = result?;
                    self.computers.push_back(computer);
                } else {
                    break;
                }
            }
        }
        Ok(value)
    }
}
