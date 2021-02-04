#![warn(rust_2018_idioms, missing_debug_implementations)]

use computer::{Code, ComputerError, OnceInput, STVirtualMachine};

pub fn result() -> Result<(), ComputerError> {
    let code = Code::from_file("day09", "input.txt")?;

    let input1 = OnceInput::new(1);
    let vm1 = STVirtualMachine::new(&code, input1);
    let result1 = vm1.get_output().get_all()?;
    println!("Day 09 - Result 1: {:?}", result1);

    let input2 = OnceInput::new(2);
    let vm2 = STVirtualMachine::new(&code, input2);
    let result2 = vm2.get_output().get_all()?;

    println!("Day 09 - Result 2: {:?}", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use computer::{ComputerError, NoInput};

    #[test]
    fn test_copy() -> Result<(), ComputerError> {
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let expected = input.clone();
        let code = input.into();
        let vm = STVirtualMachine::new(&code, NoInput {});
        let result = vm.get_output().get_all()?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_large() -> Result<(), ComputerError> {
        let input = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let code = input.into();
        let vm = STVirtualMachine::new(&code, NoInput {});
        let result = vm.get_output().get_all()?;
        let expected = vec![1219070632396864];
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_large_2() -> Result<(), ComputerError> {
        let input = vec![104, 1125899906842624, 99];
        let code = input.into();
        let vm = STVirtualMachine::new(&code, NoInput {});
        let result = vm.get_output().get_all()?;
        let expected = vec![1125899906842624];
        assert_eq!(result, expected);

        Ok(())
    }
}
