use asteroids::Asteroids;
use common::{read_all_lines, CommonError};

mod asteroids;

pub fn result() -> Result<(), CommonError> {
    let input = read_all_lines("day10", "input.txt")?;
    let asteroids = Asteroids::parse(&input);

    let (center, result1) = asteroids.get_best_position()?;
    println!("Day 10 - Result 1: {}", result1);
    println!("Day 10 - Result 1: {}", center);

    if let Some(lasered) = asteroids.get_lasered_asteroids(center).get(199) {
        let result2 = lasered.x() * 100 - lasered.y();
        println!("Day 10 - Result 2: {}", result2);
        Ok(())
    } else {
        Err(CommonError::MessageError(String::from(
            "Did not get a result",
        )))
    }
}
