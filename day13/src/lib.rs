#![warn(rust_2018_idioms, missing_debug_implementations)]
use computer::Code;
use error::GameError;
use game::{Game, Tile};

mod error;
mod game;

pub fn result() -> Result<(), GameError> {
    let code = Code::from_file("day13", "input.txt")?;
    let game = Game::paint_board(&code)?;
    let result = game.count_type(Tile::Block);
    println!("Day 13 - Result 1: {}", result);

    let result2 = Game::free_game(&code)?;
    println!("Day 13 - Result 2: {}", result2);

    Ok(())
}
