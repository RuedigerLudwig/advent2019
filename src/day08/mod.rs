use crate::common::file::read_data;
use picture::Picture;

mod picture;

pub fn result() -> Result<(), std::io::Error> {
    let input = read_data("day08", "input.txt")?;
    let picture = Picture::new(&input, 25, 6);

    if let Some(result) = picture.get_magic_number() {
        println!("Day 08 - Result 1: {}", result);
    } else {
        panic!("There is no magic number in my picture");
    }

    println!("Day 08 - Result 2:\n{}", picture);

    Ok(())
}
