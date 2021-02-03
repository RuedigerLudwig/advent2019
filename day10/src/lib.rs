#![warn(rust_2018_idioms, missing_debug_implementations)]
mod asteroids;
mod error;

use asteroids::Asteroids;
use common::read_all_lines;
use error::AsteroidError;

pub fn result() -> Result<(), AsteroidError> {
    let input = read_all_lines("day10", "input.txt")?;
    let asteroids = Asteroids::parse(&input);

    let (center, result1) = asteroids.get_best_position()?;
    println!("Day 10 - Result 1: {}", result1);

    if let Some(lasered) = asteroids.get_lasered_asteroids(center).get(199) {
        let result2 = lasered.x() * 100 - lasered.y();
        println!("Day 10 - Result 2: {}", result2);
    } else {
        println!("Day 10 - Result 2: Did not get a result");
    }
    Ok(())
}
