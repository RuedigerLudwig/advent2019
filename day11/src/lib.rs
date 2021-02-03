#![warn(rust_2018_idioms, missing_debug_implementations)]
mod error;
mod interface;
mod paint_bot;

use computer::Code;
use error::PaintError;
use interface::ComputerInterface;
use paint_bot::Bot;

pub fn result() -> Result<(), PaintError> {
    let code = Code::from_file("day11", "input.txt")?;
    let interface = ComputerInterface::new(&code);
    let mut bot = Bot::new(interface);
    bot.run(interface::Color::Black)?;

    println!("Day 11 - Result 1: {}", bot.count_painted_boards());

    let interface2 = ComputerInterface::new(&code);
    let mut bot2 = Bot::new(interface2);
    bot2.run(interface::Color::White)?;

    println!("Day 11 - Result 2:\n{}", bot2);

    Ok(())
}
