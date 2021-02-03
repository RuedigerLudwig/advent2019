#![warn(rust_2018_idioms, missing_debug_implementations)]
use common::{error::CommonError, read_single_line};
use picture::Picture;

mod picture;

pub fn result() -> Result<(), CommonError> {
    let input = read_single_line("day08", "input.txt")?;
    let picture = Picture::new(&input, 25, 6);

    if let Some(result) = picture.get_magic_number() {
        println!("Day 08 - Result 1: {}", result);
    } else {
        panic!("There is no magic number in my picture");
    }

    println!("Day 08 - Result 2:\n{}", picture);

    Ok(())
}
