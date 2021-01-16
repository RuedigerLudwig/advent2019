use computer::Code;
use game::{Game, Tile};
use std::error::Error;

mod game;

pub fn result() -> Result<(), Box<dyn Error>> {
    let code = Code::from_file("day13", "input.txt")?;
    let game = Game::paint_board(&code)?;
    let result = game.count_type(Tile::Block);
    println!("Day 13 - Result 1: {}", result);

    let result2 = Game::free_game(&code)?;
    println!("Day 13 - Result 2: {}", result2);

    Ok(())
}
