use computer::{computer_error::ComputerError, Computer};
use game::{Game, Tile};

mod game;

pub fn result() -> Result<(), ComputerError> {
    let template = Computer::from_file("day13", "input.txt")?;
    let game = Game::parse(&template)?;
    let result = game.count_type(Tile::Block);
    println!("Day 13 - Result 1: {}", result);

    let result2 = Game::free_game(&template)?;
    println!("Day 13 - Result 2: {}", result2);

    Ok(())
}
