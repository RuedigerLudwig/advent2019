use crate::ComputerError;
use common::read_single_line;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct Code(HashMap<usize, i64>);

impl Code {
    pub fn from_file(module: &str, file: &str) -> Result<Code, ComputerError> {
        read_single_line(module, file)?.parse()
    }

    pub fn get(&self) -> HashMap<usize, i64> {
        self.0.clone()
    }
}

impl<T> From<T> for Code
where
    T: IntoIterator<Item = i64>,
{
    fn from(code: T) -> Self {
        Code(code.into_iter().enumerate().collect())
    }
}

impl FromStr for Code {
    type Err = ComputerError;

    fn from_str(input: &str) -> Result<Code, Self::Err> {
        Ok(input
            .split(",")
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<_>, _>>()?
            .into())
    }
}
