use std::fs;

pub use common_error::CommonError;
pub mod common_error;
pub mod helpers;

pub fn work_on_file<F, T>(path: &str, fun: F) -> Result<Vec<T>, CommonError>
where
    F: Fn(&str) -> Result<T, CommonError>,
{
    let lines = fs::read_to_string(path)?;
    Ok(lines.lines().map(fun).collect::<Result<_, _>>()?)
}

pub fn read_all_lines(path: &str) -> Result<Vec<String>, CommonError> {
    let lines = fs::read_to_string(path)?;
    Ok(lines.lines().map(String::from).collect())
}

pub fn read_single_line(path: &str) -> Result<String, CommonError> {
    let lines = fs::read_to_string(path)?;

    let result = match lines.lines().next() {
        Some(line) => String::from(line.trim()),
        None => String::from(""),
    };
    Ok(result)
}

pub fn as_int(input: &str) -> Result<i32, CommonError> {
    Ok(input.parse()?)
}
