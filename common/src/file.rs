use std::{fs, io};

pub fn read_data(module: &str, file: &str) -> io::Result<String> {
    fs::read_to_string(format!("{}/data/{}", module, file))
        .or_else(|_| fs::read_to_string(format!("data/{}", file)))
}
