use std::fmt::Display;

use crate::CommonError;

pub fn as_int(input: &str) -> Result<i32, CommonError> {
    Ok(input.parse()?)
}

pub fn as_long(input: &str) -> Result<i64, CommonError> {
    Ok(input.parse()?)
}

pub fn join<T: Display>(lst: &[T], sep: &str) -> String {
    lst.iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(sep)
}
