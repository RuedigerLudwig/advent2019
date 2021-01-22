use std::fs;

use crate::CommonError;

fn read_from_file(module: &str, file: &str) -> Result<String, CommonError> {
    let mut result = fs::read_to_string(format!("{}/data/{}", module, file));
    if result.is_err() {
        result = fs::read_to_string(format!("data/{}", file));
    }
    Ok(result?)
}

pub fn read_all_lines(module: &str, file: &str) -> Result<Vec<String>, CommonError> {
    let lines = read_from_file(module, file)?;
    Ok(lines.lines().map(String::from).collect())
}

pub fn read_as_string(module: &str, file: &str) -> Result<String, CommonError> {
    let lines = read_from_file(module, file)?;
    Ok(lines)
}

pub fn read_single_line(module: &str, file: &str) -> Result<String, CommonError> {
    let lines = read_from_file(module, file)?;
    Ok(lines.lines().next().unwrap_or_default().to_owned())
}
