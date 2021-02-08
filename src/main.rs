#![feature(iter_map_while)]
#![warn(rust_2018_idioms, missing_debug_implementations)]
mod common;
mod computer;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod macros;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let days = env::args().skip(1).collect::<Vec<_>>();

    if days.is_empty() || days.contains(&"day01".into()) {
        day01::result()?;
    }

    if days.is_empty() || days.contains(&"day02".into()) {
        day02::result()?;
    }

    if days.is_empty() || days.contains(&"day03".into()) {
        day03::result()?;
    }

    if days.is_empty() || days.contains(&"day04".into()) {
        day04::result()?;
    }

    if days.is_empty() || days.contains(&"day05".into()) {
        day05::result()?;
    }

    if days.is_empty() || days.contains(&"day06".into()) {
        day06::result()?;
    }

    if days.is_empty() || days.contains(&"day07".into()) {
        day07::result()?;
    }

    if days.is_empty() || days.contains(&"day08".into()) {
        day08::result()?;
    }

    if days.is_empty() || days.contains(&"day09".into()) {
        day09::result()?;
    }

    if days.is_empty() || days.contains(&"day10".into()) {
        day10::result()?;
    }

    if days.is_empty() || days.contains(&"day11".into()) {
        day11::result()?;
    }

    if days.is_empty() || days.contains(&"day12".into()) {
        day12::result()?;
    }

    if days.is_empty() || days.contains(&"day13".into()) {
        day13::result()?;
    }

    if days.is_empty() || days.contains(&"day14".into()) {
        day14::result()?;
    }

    if days.is_empty() || days.contains(&"day15".into()) {
        day15::result()?;
    }

    if days.is_empty() || days.contains(&"day16".into()) {
        day16::result()?;
    }

    if days.is_empty() || days.contains(&"day17".into()) {
        day17::result()?;
    }

    if days.is_empty() || days.contains(&"day18".into()) {
        day18::result()?;
    }

    if days.is_empty() || days.contains(&"day19".into()) {
        day19::result()?;
    }

    if days.is_empty() || days.contains(&"day20".into()) {
        day20::result()?;
    }

    if days.is_empty() || days.contains(&"day21".into()) {
        day21::result()?;
    }

    if days.is_empty() || days.contains(&"day22".into()) {
        day22::result()?;
    }

    if days.is_empty() || days.contains(&"day23".into()) {
        day23::result()?;
    }

    if days.is_empty() || days.contains(&"day24".into()) {
        day24::result()?;
    }

    if days.is_empty() || days.contains(&"day25".into()) {
        day25::result()?;
    }

    if days.len() == 1 && days.contains(&"game".into()) {
        day25::interactive()?;
    }

    Ok(())
}
