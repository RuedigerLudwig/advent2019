use crate::computer::{Code, ComputerError, OnceInput, VirtualMachine};

pub fn result() -> Result<(), ComputerError> {
    let code = Code::from_file("day05", "input.txt")?;

    let mut vm1 = VirtualMachine::new(code.clone(), OnceInput::new(1));
    let result1 = vm1.get_all()?;

    println!("Day 05 - Result 1: {:?}", result1);

    let mut vm2 = VirtualMachine::new(code, OnceInput::new(5));
    let result2 = vm2.get_all()?;

    println!("Day 05 - Result 2: {:?}", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::computer::{ComputerError, NoInput};

    use super::*;

    #[test]
    fn param_test() -> Result<(), ComputerError> {
        let code = vec![1002, 4, 3, 4, 33].into();
        let mut vm = VirtualMachine::new(code, NoInput {});
        vm.get_all()?;

        let result = vm.get_memory();

        let expected = vec![1002, 4, 3, 4, 99];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn negative_test() -> Result<(), ComputerError> {
        let code = "1101,100,-1,4,0".parse()?;
        let mut vm = VirtualMachine::new(code, NoInput {});
        vm.get_all()?;
        let result = vm.get_memory();

        let expected = vec![1101, 100, -1, 4, 99];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn input_test() -> Result<(), ComputerError> {
        let code = vec![3, 3, 99, 0].into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(123));
        vm.get_all()?;
        let result = vm.get_memory();

        let expected = vec![3, 3, 99, 123];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn io_test() -> Result<(), ComputerError> {
        let code = vec![3, 0, 4, 0, 99].into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(123));
        let result = vm.get_all()?;

        let expected = vec![123];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_eq_pos_eq() -> Result<(), ComputerError> {
        let code = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(8));
        let result = vm.get_all()?;

        let expected = vec![1];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_eq_pos_ne() -> Result<(), ComputerError> {
        let code = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(9));
        let result = vm.get_all()?;

        let expected = vec![0];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_lt_pos_eq() -> Result<(), ComputerError> {
        let code = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(7));
        let result = vm.get_all()?;

        let expected = vec![1];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_lt_pos_ne() -> Result<(), ComputerError> {
        let code = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(9));
        let result = vm.get_all()?;

        let expected = vec![0];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_eq_imm_eq() -> Result<(), ComputerError> {
        let code = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99].into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(8));
        let result = vm.get_all()?;

        let expected = vec![1];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_eq_imm_ne() -> Result<(), ComputerError> {
        let code = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99].into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(9));
        let result = vm.get_all()?;

        let expected = vec![0];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_lt_imm_eq() -> Result<(), ComputerError> {
        let code = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99].into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(7));
        let result = vm.get_all()?;

        let expected = vec![1];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn is_lt_imm_ne() -> Result<(), ComputerError> {
        let code = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99].into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(9));
        let result = vm.get_all()?;

        let expected = vec![0];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn long_lt() -> Result<(), ComputerError> {
        let code = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]
        .into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(7));
        let result = vm.get_all()?;

        let expected = vec![999];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn long_eq() -> Result<(), ComputerError> {
        let code = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]
        .into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(8));
        let result = vm.get_all()?;

        let expected = vec![1000];

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn long_gt() -> Result<(), ComputerError> {
        let code = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]
        .into();
        let mut vm = VirtualMachine::new(code, OnceInput::new(9));
        let result = vm.get_all()?;

        let expected = vec![1001];

        assert_eq!(result, expected);

        Ok(())
    }
}
