mod spring;
mod spring_error;

use std::error::Error;

use computer::Code;
use spring::{Instruction, ReadRegister, SpringBotComputer, WriteRegister};

use Instruction::*;
use ReadRegister::*;
use WriteRegister::*;

pub fn result() -> Result<(), Box<dyn Error>> {
    let code = Code::from_file("day21", "input.txt")?;
    let mut springbot = SpringBotComputer::new(&code);

    let instructions1 = vec![NOT(C, JOut), AND(D, JOut), NOT(A, TOut), OR(T, JOut)];
    let result1 = springbot.walk(&instructions1, true)?;
    println!("Day 21 - Result 1: {}", result1);

    let instructions2 = vec![
        NOT(B, TOut),
        NOT(C, JOut),
        OR(T, JOut),
        AND(H, JOut),
        AND(D, JOut),
        NOT(A, TOut),
        OR(T, JOut),
    ];
    let result2 = springbot.run(&instructions2, true)?;
    println!("Day 21 - Result 2: {}", result2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() -> Result<(), Box<dyn Error>> {
        let code = Code::from_file("day21", "input.txt")?;
        let mut springbot = SpringBotComputer::new(&code);

        let instructions1 = vec![NOT(C, JOut), AND(D, JOut), NOT(A, TOut), OR(T, JOut)];
        springbot.walk(&instructions1, false)?;

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn Error>> {
        let code = Code::from_file("day21", "input.txt")?;
        let mut springbot = SpringBotComputer::new(&code);

        let instructions = vec![
            NOT(B, TOut),
            NOT(C, JOut),
            OR(T, JOut),
            AND(D, JOut),
            AND(H, JOut),
            NOT(A, TOut),
            OR(T, JOut),
        ];
        springbot.run(&instructions, false)?;

        Ok(())
    }
}
