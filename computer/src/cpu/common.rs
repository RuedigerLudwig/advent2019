use std::collections::HashMap;

use crate::ComputerError;

use super::modes::AddrModes;

pub fn analyze_instruction(instruction: i64) -> Result<(u8, AddrModes), ComputerError> {
    let op_code = (instruction % 100) as u8;
    Ok((op_code, AddrModes::new(instruction / 100)?))
}

pub fn disassemble(
    code: &HashMap<usize, i64>,
    pointer: usize,
) -> Result<(String, usize), ComputerError> {
    let max = code.keys().max();
    let instruction = code.get(&pointer);

    if let Some((max, instruction)) = max.zip(instruction) {
        let width = ((16 - (max.leading_zeros() + 3) / 4) as usize).max(4);

        if let Ok((opcode, modes)) = analyze_instruction(*instruction) {
            let (inst, num_params) = match opcode {
                1 => ("ADD", 3),
                2 => ("MUL", 3),
                3 => ("INP", 1),
                4 => ("OUT", 1),
                5 => ("JNZ", 2),
                6 => ("JZ", 2),
                7 => ("LT", 3),
                8 => ("EQ", 3),
                9 => ("OFF", 1),
                99 => ("STP", 0),
                _ => ("???", 1),
            };
            let params = (0..num_params)
                .map(|num| {
                    if let Some(p) = code.get(&(pointer + num + 1)) {
                        modes.get(num).format(p, width)
                    } else {
                        String::from("?").repeat(width + 1)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");

            let output = format!("{:3} {}", inst, params);
            Ok((output, num_params + 1))
        } else {
            let output = format!("Value: {}", instruction);

            Ok((output, 1))
        }
    } else {
        Err(ComputerError::CanNotDisassemble)
    }
}
