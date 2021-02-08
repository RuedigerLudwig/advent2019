use crate::computer::{Code, NoInput, VirtualMachine};

use super::error::SimpleError;

pub fn run_patched(code: Code) -> Result<i64, SimpleError> {
    let mut vm = VirtualMachine::new(code, NoInput {});

    vm.patch_memory(1, 12);
    vm.patch_memory(2, 2);
    vm.get_all()?;

    Ok(vm.get_memory()[0])
}

pub fn find_numbers(code: Code) -> Result<(i64, i64), SimpleError> {
    let mut vm = VirtualMachine::new(code.clone(), NoInput {});
    for noun in 0..100 {
        for verb in 0..100 {
            vm.restart();
            vm.patch_memory(1, noun);
            vm.patch_memory(2, verb);
            vm.get_all()?;

            if vm.get_memory()[0] == 19690720 {
                return Ok((noun, verb));
            }
        }
    }
    Err(SimpleError::NoNumbersFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<(), SimpleError> {
        let code = "1,9,10,3,2,3,11,0,99,30,40,50".parse()?;
        let vm = VirtualMachine::new(code, NoInput {});
        let result = vm.get_memory();
        let expected = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_computer_running() -> Result<(), SimpleError> {
        let code = "1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50".parse()?;
        let mut vm = VirtualMachine::new(code, NoInput {});
        vm.get_all()?;
        let result = vm.get_memory();

        let expected: Vec<i64> = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(result, expected);

        Ok(())
    }
}
