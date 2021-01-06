use crate::CommonError;

pub fn as_int(input: &str) -> Result<i32, CommonError> {
    Ok(input.parse()?)
}

pub fn as_long(input: &str) -> Result<i64, CommonError> {
    Ok(input.parse()?)
}
