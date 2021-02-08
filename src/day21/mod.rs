mod error;
mod spring;

use crate::computer::Code;
use error::SpringError;
use spring::{Instruction, Read, SpringBotComputer, Write};

use Instruction::*;

pub fn result() -> Result<(), SpringError> {
    let code = Code::from_file("day21", "input.txt")?;
    let mut springbot = SpringBotComputer::new(code);

    let instructions1 = vec![
        NOT(Read::C, Write::J),
        AND(Read::D, Write::J),
        NOT(Read::A, Write::T),
        OR(Read::T, Write::J),
    ];
    let result1 = springbot.walk(&instructions1, true)?;
    println!("Day 21 - Result 1: {}", result1);

    let instructions2 = vec![
        NOT(Read::B, Write::T),
        NOT(Read::C, Write::J),
        OR(Read::T, Write::J),
        AND(Read::H, Write::J),
        AND(Read::D, Write::J),
        NOT(Read::A, Write::T),
        OR(Read::T, Write::J),
    ];
    let result2 = springbot.run(&instructions2, true)?;
    println!("Day 21 - Result 2: {}", result2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() -> Result<(), SpringError> {
        let code = Code::from_file("day21", "input.txt")?;
        let mut springbot = SpringBotComputer::new(code);

        let instructions1 = vec![
            NOT(Read::C, Write::J),
            AND(Read::D, Write::J),
            NOT(Read::A, Write::T),
            OR(Read::T, Write::J),
        ];
        springbot.walk(&instructions1, false)?;

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), SpringError> {
        let code = Code::from_file("day21", "input.txt")?;
        let mut springbot = SpringBotComputer::new(code);

        let instructions = vec![
            NOT(Read::B, Write::T),
            NOT(Read::C, Write::J),
            OR(Read::T, Write::J),
            AND(Read::D, Write::J),
            AND(Read::H, Write::J),
            NOT(Read::A, Write::T),
            OR(Read::T, Write::J),
        ];
        springbot.run(&instructions, false)?;

        Ok(())
    }
}
