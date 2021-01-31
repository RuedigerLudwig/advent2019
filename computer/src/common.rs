use std::collections::HashMap;

use crate::{modes::AddrModes, ComputerError};

pub fn analyze_instruction(instruction: i64) -> Result<(u8, AddrModes), ComputerError> {
    let op_code = (instruction % 100) as u8;
    Ok((op_code, AddrModes::new(instruction / 100)?))
}

pub fn disassemble(code: &HashMap<usize, i64>, pointer: usize) -> (String, usize) {
    let max = *code.keys().max().unwrap();
    let width = ((16 - (max.leading_zeros() + 3) / 4) as usize).max(4);

    let instruction = code.get(&pointer).unwrap();
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
        (output, num_params + 1)
    } else {
        let output = format!("Value: {}", instruction);

        (output, 1)
    }
}
