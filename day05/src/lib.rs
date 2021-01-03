use common::read_single_line;
use computer::{computer_error::ComputerError, Computer};
use std::str::FromStr;

pub fn result1() -> Result<String, ComputerError> {
    let input = read_single_line("day05", "input.txt")?;
    let mut computer = Computer::from_str(&input)?;

    computer.provide_input(1);
    let result = computer.run()?;

    Ok(format!("Day 05 - Result 1: {:?}", result))
}

pub fn result2() -> Result<String, ComputerError> {
    let input = read_single_line("day05", "input.txt")?;
    let mut computer = Computer::from_str(&input)?;

    computer.provide_input(5);
    let result = computer.run()?;

    Ok(format!("Day 05 - Result 2: {:?}", result))
}

#[cfg(test)]
mod tests {
    use computer::{computer_error::ComputerError, Computer};
    use std::str::FromStr;

    #[test]
    fn param_test() -> Result<(), ComputerError> {
        let input = vec![1002, 4, 3, 4, 33];
        let mut computer = Computer::new(input)?;
        computer.run()?;
        let result = computer.get_memory();

        let expected: Vec<i32> = vec![1002, 4, 3, 4, 99];

        assert_eq!(result, &expected);

        Ok(())
    }

    #[test]
    fn negative_test() -> Result<(), ComputerError> {
        let input = "1101,100,-1,4,0";
        let mut computer = Computer::from_str(input)?;
        computer.run()?;
        let result = computer.get_memory();

        let expected: Vec<i32> = vec![1101, 100, -1, 4, 99];

        assert_eq!(result, &expected);

        Ok(())
    }

    #[test]
    fn input_test() -> Result<(), ComputerError> {
        let input = vec![3, 3, 99, 0];
        let mut computer = Computer::new(input)?;
        computer.provide_input(123);
        computer.run()?;
        let result = computer.get_memory();

        let expected: Vec<i32> = vec![3, 3, 99, 123];

        assert_eq!(result, &expected);

        Ok(())
    }

    #[test]
    fn io_test() -> Result<(), ComputerError> {
        let input = vec![3, 0, 4, 0, 99];
        let mut computer = Computer::new(input)?;
        computer.provide_input(123);
        let result = computer.run()?;

        let expected: Vec<i32> = vec![123];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_eq_pos_eq() -> Result<(), ComputerError> {
        let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut computer = Computer::new(input)?;
        computer.provide_input(8);
        let result = computer.run()?;

        let expected: Vec<i32> = vec![1];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_eq_pos_ne() -> Result<(), ComputerError> {
        let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut computer = Computer::new(input)?;
        computer.provide_input(9);
        let result = computer.run()?;

        let expected: Vec<i32> = vec![0];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_lt_pos_eq() -> Result<(), ComputerError> {
        let input = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut computer = Computer::new(input)?;
        computer.provide_input(7);
        let result = computer.run()?;

        let expected: Vec<i32> = vec![1];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_lt_pos_ne() -> Result<(), ComputerError> {
        let input = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut computer = Computer::new(input)?;
        computer.provide_input(9);
        let result = computer.run()?;

        let expected: Vec<i32> = vec![0];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_eq_imm_eq() -> Result<(), ComputerError> {
        let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut computer = Computer::new(input)?;
        computer.provide_input(8);
        let result = computer.run()?;

        let expected: Vec<i32> = vec![1];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_eq_imm_ne() -> Result<(), ComputerError> {
        let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut computer = Computer::new(input)?;
        computer.provide_input(9);
        let result = computer.run()?;

        let expected: Vec<i32> = vec![0];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_lt_imm_eq() -> Result<(), ComputerError> {
        let input = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut computer = Computer::new(input)?;
        computer.provide_input(7);
        let result = computer.run()?;

        let expected: Vec<i32> = vec![1];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_lt_imm_ne() -> Result<(), ComputerError> {
        let input = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut computer = Computer::new(input)?;
        computer.provide_input(9);
        let result = computer.run()?;

        let expected: Vec<i32> = vec![0];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn long_lt() -> Result<(), ComputerError> {
        let input = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut computer = Computer::new(input)?;
        computer.provide_input(7);
        let result = computer.run()?;

        let expected: Vec<i32> = vec![999];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn long_eq() -> Result<(), ComputerError> {
        let input = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut computer = Computer::new(input)?;
        computer.provide_input(8);
        let result = computer.run()?;

        let expected: Vec<i32> = vec![1000];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn long_gt() -> Result<(), ComputerError> {
        let input = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut computer = Computer::new(input)?;
        computer.provide_input(9);
        let result = computer.run()?;

        let expected: Vec<i32> = vec![1001];

        assert_eq!(result, expected);

        Ok(())
    }
}
