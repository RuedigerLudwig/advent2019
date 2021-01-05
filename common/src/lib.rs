use std::fs;

pub use common_error::CommonError;
pub mod common_error;
pub mod helpers;

fn read_from_file(module: &str, file: &str) -> Result<String, CommonError> {
    let mut result = fs::read_to_string(format!("{}/data/{}", module, file));
    if result.is_err() {
        result = fs::read_to_string(format!("data/{}", file));
    }
    Ok(result?)
}

pub fn work_on_file<F, T>(module: &str, file: &str, fun: F) -> Result<Vec<T>, CommonError>
where
    F: Fn(&str) -> Result<T, CommonError>,
{
    let lines = read_from_file(module, file)?;
    Ok(lines.lines().map(fun).collect::<Result<_, _>>()?)
}

pub fn read_all_lines(module: &str, file: &str) -> Result<Vec<String>, CommonError> {
    let lines = read_from_file(module, file)?;
    Ok(lines.lines().map(String::from).collect())
}

pub fn read_single_line(module: &str, file: &str) -> Result<String, CommonError> {
    let lines = read_from_file(module, file)?;

    let result = match lines.lines().next() {
        Some(line) => String::from(line.trim()),
        None => String::from(""),
    };
    Ok(result)
}

pub fn as_int(input: &str) -> Result<i32, CommonError> {
    Ok(input.parse()?)
}

pub fn as_long(input: &str) -> Result<i64, CommonError> {
    Ok(input.parse()?)
}
