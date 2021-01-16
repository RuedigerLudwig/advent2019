use common::{as_long, read_single_line, CommonError};
use std::str::FromStr;

#[derive(Debug)]
pub struct Code {
    _code: Vec<i64>,
}

impl Code {
    pub fn from_file(module: &str, file: &str) -> Result<Code, CommonError> {
        let input = read_single_line(module, file)?;
        input.parse()
    }

    pub fn get<'a>(&'a self) -> &'a [i64] {
        &self._code
    }
}

impl From<Vec<i64>> for Code {
    fn from(code: Vec<i64>) -> Self {
        Code { _code: code }
    }
}

impl FromStr for Code {
    type Err = CommonError;

    fn from_str(input: &str) -> Result<Code, Self::Err> {
        let code: Vec<_> = input.split(",").map(as_long).collect::<Result<_, _>>()?;
        Ok(code.into())
    }
}
