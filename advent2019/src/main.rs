use day01;
use day02;
use std::env;

fn main() {
    let days: Vec<String> = env::args().skip(1).collect();

    if days.is_empty() || days.contains(&String::from("day01")) {
        day01::result1();
        day01::result2();
    }

    if days.is_empty() || days.contains(&String::from("day02")) {
        day02::result1();
        day02::result2();
    }
}
