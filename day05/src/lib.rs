use std::error::Error;

use computer::{Code, ListInput, VirtualMachine};

pub fn result() -> Result<(), Box<dyn Error>> {
    let code = Code::from_file("day05", "input.txt")?;

    let vm1 = VirtualMachine::new(&code, &ListInput::single(1));
    let result1 = vm1.get_output().get_all()?;

    println!("Day 05 - Result 1: {:?}", result1);

    let vm2 = VirtualMachine::new(&code, &ListInput::single(5));
    let result2 = vm2.get_output().get_all()?;

    println!("Day 05 - Result 2: {:?}", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use computer::{ComputerError, NoInput};

    use super::*;

    #[test]
    fn param_test() -> Result<(), Box<dyn Error>> {
        let input = vec![1002, 4, 3, 4, 33];
        let code = input.into();
        let vm = VirtualMachine::new(&code, &NoInput {});
        vm.get_output().get_all()?;

        let result = vm.get_memory();

        let expected = vec![1002, 4, 3, 4, 99];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn negative_test() -> Result<(), Box<dyn Error>> {
        let input = "1101,100,-1,4,0";
        let code: Code = input.parse()?;
        let vm = VirtualMachine::new(&code, &NoInput {});
        vm.get_output().get_all()?;
        let result = vm.get_memory();

        let expected = vec![1101, 100, -1, 4, 99];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn input_test() -> Result<(), ComputerError> {
        let input = vec![3, 3, 99, 0];
        let code = input.into();
        let input = ListInput::single(123);
        let vm = VirtualMachine::new(&code, &input);
        vm.get_output().get_all()?;
        let result = vm.get_memory();

        let expected = vec![3, 3, 99, 123];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn io_test() -> Result<(), ComputerError> {
        let input = vec![3, 0, 4, 0, 99];
        let code = input.into();
        let input = ListInput::single(123);
        let vm = VirtualMachine::new(&code, &input);
        let result = vm.get_output().get_all()?;

        let expected = vec![123];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_eq_pos_eq() -> Result<(), ComputerError> {
        let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let code = input.into();
        let input = ListInput::single(8);
        let vm = VirtualMachine::new(&code, &input);
        let result = vm.get_output().get_all()?;

        let expected = vec![1];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_eq_pos_ne() -> Result<(), ComputerError> {
        let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let code = input.into();
        let input = ListInput::single(9);
        let vm = VirtualMachine::new(&code, &input);
        let result = vm.get_output().get_all()?;

        let expected = vec![0];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_lt_pos_eq() -> Result<(), ComputerError> {
        let input = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let code = input.into();
        let input = ListInput::single(7);
        let vm = VirtualMachine::new(&code, &input);
        let result = vm.get_output().get_all()?;

        let expected = vec![1];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_lt_pos_ne() -> Result<(), ComputerError> {
        let input = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let code = input.into();
        let input = ListInput::single(9);
        let vm = VirtualMachine::new(&code, &input);
        let result = vm.get_output().get_all()?;

        let expected = vec![0];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_eq_imm_eq() -> Result<(), ComputerError> {
        let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let code = input.into();
        let input = ListInput::single(8);
        let vm = VirtualMachine::new(&code, &input);
        let result = vm.get_output().get_all()?;

        let expected = vec![1];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_eq_imm_ne() -> Result<(), ComputerError> {
        let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let code = input.into();
        let input = ListInput::single(9);
        let vm = VirtualMachine::new(&code, &input);
        let result = vm.get_output().get_all()?;

        let expected = vec![0];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_lt_imm_eq() -> Result<(), ComputerError> {
        let input = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let code = input.into();
        let input = ListInput::single(7);
        let vm = VirtualMachine::new(&code, &input);
        let result = vm.get_output().get_all()?;

        let expected = vec![1];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_lt_imm_ne() -> Result<(), ComputerError> {
        let input = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let code = input.into();
        let input = ListInput::single(9);
        let vm = VirtualMachine::new(&code, &input);
        let result = vm.get_output().get_all()?;

        let expected = vec![0];

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
        let code = input.into();
        let input = ListInput::single(7);
        let vm = VirtualMachine::new(&code, &input);
        let result = vm.get_output().get_all()?;

        let expected = vec![999];

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
        let code = input.into();
        let input = ListInput::single(8);
        let vm = VirtualMachine::new(&code, &input);
        let result = vm.get_output().get_all()?;

        let expected = vec![1000];

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
        let code = input.into();
        let input = ListInput::single(9);
        let vm = VirtualMachine::new(&code, &input);
        let result = vm.get_output().get_all()?;

        let expected = vec![1001];

        assert_eq!(result, expected);

        Ok(())
    }
}
