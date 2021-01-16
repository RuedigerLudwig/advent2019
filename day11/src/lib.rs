use std::error::Error;

use computer::Code;
use interface::ComputerInterface;
use paint_bot::Bot;

mod interface;
mod paint_bot;

pub fn result() -> Result<(), Box<dyn Error>> {
    let code = Code::from_file("day11", "input.txt")?;
    let interface = ComputerInterface::new(&code);
    let mut bot = Bot::new(interface);
    bot.run()?;

    println!("Day 11 - Result 1: {}", bot.count_painted_boards());

    let interface2 = ComputerInterface::new(&code);
    let mut bot2 = Bot::new(interface2);
    bot2.paint_current_board(interface::Color::White);
    bot2.run()?;

    println!("Day 11 - Result 2:\n{}", bot2);

    Ok(())
}
