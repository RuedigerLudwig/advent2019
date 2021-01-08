use computer::{computer_error::ComputerError, Computer};

pub fn result() -> Result<(), ComputerError> {
    let template = Computer::from_file("day02", "input.txt")?;

    let mut computer1 = template.clone();

    computer1.patch_memory(1, 12);
    computer1.patch_memory(2, 2);
    computer1.run()?;

    let result1 = computer1.get_memory()[0];
    println!("Day 02 - Result 1: {}", result1);

    let computer2 = template.clone();
    let (noun, verb) = test_numbers(computer2)?;

    println!("Day 02 - Result 2: {}", 100 * noun + verb);

    Ok(())
}

fn test_numbers(original: Computer) -> Result<(i64, i64), ComputerError> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut computer = original.clone();

            computer.patch_memory(1, noun);
            computer.patch_memory(2, verb);
            computer.run()?;

            if computer.get_memory()[0] == 19690720 {
                return Ok((noun, verb));
            }
        }
    }
    Err(ComputerError::MessageError(String::from(
        "No suitable numbers found",
    )))
}

#[cfg(test)]
mod tests {
    use computer::{computer_error::ComputerError, Computer};
    use std::str::FromStr;

    #[test]
    fn test_parse() -> Result<(), ComputerError> {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let computer = Computer::from_str(&input)?;
        let result = computer.get_memory();
        let expected = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_computer_running() -> Result<(), ComputerError> {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let mut computer = Computer::new(&input);
        computer.run()?;
        let result = computer.get_memory();

        let expected: Vec<i64> = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(result, expected);

        Ok(())
    }
}
