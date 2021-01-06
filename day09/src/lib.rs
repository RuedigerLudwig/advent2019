use computer::{computer_error::ComputerError, Computer};

pub fn result() -> Result<(), ComputerError> {
    let template = Computer::from_file("day09", "input.txt")?;

    let mut computer1 = template.clone();
    computer1.provide_input(1);
    let result1 = computer1.run()?;
    println!("Day 09 - Result 1: {:?}", result1);

    let mut computer2 = template.clone();
    computer2.provide_input(2);
    let result2 = computer2.run()?;

    println!("Day 09 - Result 2: {:?}", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use computer::{computer_error::ComputerError, Computer};

    #[test]
    fn test_copy() -> Result<(), ComputerError> {
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut computer = Computer::new(&input)?;
        let result = computer.run()?;
        assert_eq!(result, input);

        Ok(())
    }

    #[test]
    fn test_large() -> Result<(), ComputerError> {
        let input = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut computer = Computer::new(&input)?;
        let result = computer.run()?;
        let expected = vec![1219070632396864];
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_large_2() -> Result<(), ComputerError> {
        let input = vec![104, 1125899906842624, 99];
        let mut computer = Computer::new(&input)?;
        let result = computer.run()?;
        let expected = vec![1125899906842624];
        assert_eq!(result, expected);

        Ok(())
    }
}
