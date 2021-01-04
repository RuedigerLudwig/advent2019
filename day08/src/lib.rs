use common::{read_single_line, CommonError};
use picture::Picture;

mod picture;

pub fn result1() -> Result<String, CommonError> {
    let input = read_single_line("day08", "input.txt")?;
    let picture = Picture::new(&input, 25, 6);

    if let Some(result) = picture.get_magic_number() {
        Ok(format!("Day 08 - Result 1: {}", result))
    } else {
        Err(CommonError::MessageError(String::from(
            "There is no magic number in my picture",
        )))
    }
}

pub fn result2() -> Result<String, CommonError> {
    let input = read_single_line("day08", "input.txt")?;
    let picture = Picture::new(&input, 25, 6);

    Ok(format!("Day 08 - Result 2:\n{}", picture))
}
