use std::error::Error;

use computer::{Code, ComputerError, VirtualMachine};

pub fn result() -> Result<(), Box<dyn Error>> {
    let code = Code::from_file("day02", "input.txt")?;

    let vm = VirtualMachine::new(&code);

    vm.patch_memory(1, 12);
    vm.patch_memory(2, 2);
    vm.get_output().get_all()?;

    let result1 = vm.get_memory()[0];
    println!("Day 02 - Result 1: {}", result1);

    let (noun, verb) = test_numbers(&code)?;

    println!("Day 02 - Result 2: {}", 100 * noun + verb);

    Ok(())
}

fn test_numbers(code: &Code) -> Result<(i64, i64), ComputerError> {
    for noun in 0..100 {
        for verb in 0..100 {
            let vm = VirtualMachine::new(&code);

            vm.patch_memory(1, noun);
            vm.patch_memory(2, verb);
            vm.get_output().get_all()?;

            if vm.get_memory()[0] == 19690720 {
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
    use super::*;

    #[test]
    fn test_parse() -> Result<(), Box<dyn Error>> {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let code: Code = input.parse()?;
        let vm = VirtualMachine::new(&code);
        let result = vm.get_memory();
        let expected = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_computer_running() -> Result<(), Box<dyn Error>> {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let code = input.into();
        let vm = VirtualMachine::new(&code);
        vm.get_output().get_all()?;
        let result = vm.get_memory();

        let expected: Vec<i64> = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(result, expected);

        Ok(())
    }
}
