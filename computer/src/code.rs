use common::{as_long, read_single_line, CommonError};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
pub struct Code {
    _code: HashMap<usize, i64>,
}

impl Code {
    pub fn from_file(module: &str, file: &str) -> Result<Code, CommonError> {
        let input = read_single_line(module, file)?;
        input.parse()
    }

    pub fn get<'a>(&'a self) -> &'a HashMap<usize, i64> {
        &self._code
    }
}

impl From<HashMap<usize, i64>> for Code {
    fn from(code: HashMap<usize, i64>) -> Self {
        Code { _code: code }
    }
}

impl From<Vec<i64>> for Code {
    fn from(code: Vec<i64>) -> Self {
        Code {
            _code: code.iter().copied().enumerate().collect(),
        }
    }
}

impl FromStr for Code {
    type Err = CommonError;

    fn from_str(input: &str) -> Result<Code, Self::Err> {
        let _code: HashMap<_, _> = input
            .split(",")
            .enumerate()
            .map(|(pos, s)| as_long(s).map(|l| (pos, l)))
            .collect::<Result<_, _>>()?;

        Ok(Code { _code })
    }
}
