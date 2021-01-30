use day01;
use day02;
use day03;
use day04;
use day05;
use day06;
use day07;
use day08;
use day09;
use day10;
use day11;
use day12;
use day13;
use day14;
use day15;
use day16;
use day17;
use day18;
use day19;
use day20;
use day21;
use day22;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let days: Vec<String> = env::args().skip(1).collect();

    if days.is_empty() || days.contains(&String::from("day01")) {
        day01::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day02")) {
        day02::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day03")) {
        day03::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day04")) {
        day04::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day05")) {
        day05::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day06")) {
        day06::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day07")) {
        day07::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day08")) {
        day08::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day09")) {
        day09::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day10")) {
        day10::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day11")) {
        day11::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day12")) {
        day12::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day13")) {
        day13::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day14")) {
        day14::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day15")) {
        day15::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day16")) {
        day16::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day17")) {
        day17::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day18")) {
        day18::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day19")) {
        day19::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day20")) {
        day20::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day21")) {
        day21::result()?;
    }

    if days.is_empty() || days.contains(&String::from("day22")) {
        day22::result()?;
    }

    Ok(())
}
