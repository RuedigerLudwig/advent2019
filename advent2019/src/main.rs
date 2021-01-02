use day01;
use day02;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let days: Vec<String> = env::args().skip(1).collect();

    if days.is_empty() || days.contains(&String::from("day01")) {
        println!("{}", day01::result1()?);
        println!("{}", day01::result2()?);
    }

    if days.is_empty() || days.contains(&String::from("day02")) {
        println!("{}", day02::result1()?);
        println!("{}", day02::result2()?);
    }

    Ok(())
}
