use std::collections::HashMap;

use super::{debug_codes, modes::AddrModes, operation_result::OperationResult, Cpu};
use crate::ComputerError;
use common::convert::join;

pub struct DebugInfo<'a> {
    cpu: &'a Cpu<'a>,
    id: Option<usize>,
    offset: i64,

    name: &'a str,
    range: (usize, usize),
    modes: &'a AddrModes,
    info_text: &'a str,
    outcome: OperationResult,

    processed_params: Option<&'a [i64]>,

    write: Option<(i64, usize)>,
}

impl<'a> DebugInfo<'a> {
    pub fn new(
        cpu: &'a Cpu<'a>,
        name: &'a str,
        range: (usize, usize),
        modes: &'a AddrModes,
        info_text: &'a str,
        outcome: OperationResult,
    ) -> DebugInfo<'a> {
        DebugInfo {
            cpu,
            offset: cpu.get_offset(),
            id: cpu.get_id(),
            name,
            range,
            modes,
            info_text,
            outcome,
            processed_params: None,
            write: None,
        }
    }

    pub fn add_params(mut self, processed_params: &'a [i64]) -> Self {
        self.processed_params = Some(processed_params);
        self
    }

    pub fn add_write(mut self, write_result: i64, write_addr: usize) -> Self {
        self.write = Some((write_result, write_addr));
        self
    }

    pub fn print(&self) -> Result<(), ComputerError> {
        if self.cpu.get_debug_level() & debug_codes::HEAD != 0 {
            if let Some(id) = self.id {
                println!(
                    "({}-{}) | {{{}}} | \"{}\"",
                    self.range.0,
                    self.range.1 - 1,
                    self.offset,
                    id
                );
            } else {
                println!(
                    "({}-{}) | {{{}}}",
                    self.range.0,
                    self.range.1 - 1,
                    self.offset
                );
            }
        }

        if self.cpu.get_debug_level() & debug_codes::DISASSEMBLE != 0 {
            let (output, _) = DebugInfo::disassemble(self.cpu.get_memory(), self.range.0)?;
            println!("    {}", output);
        }

        if self.cpu.get_debug_level() & debug_codes::FULL_INFO != 0 {
            println!("    Info: {:3}", self.name);
            println!(
                "          Inst:    {}",
                self.modes[0].format(self.cpu.get_direct_value(self.range.0)?, 4)
            );
            print!("          Raw:     ");
            for ad in self.range.0 + 1..self.range.1 {
                let mode = self.modes[ad - self.range.0];
                print!("{} ", mode.format(self.cpu.get_direct_value(ad)?, 4));
            }
            println!();
            if let Some(processed_params) = self.processed_params {
                println!("          Params:  {}", join(processed_params, ", "));
            }
            if let Some((result, addr)) = self.write {
                println!("          Write    {} to {}", result, addr);
            }
            println!("          Result:  {}", self.outcome);
        }

        if self.cpu.get_debug_level() & debug_codes::TEXT_INFO != 0 {
            println!("    {}", self.info_text);
        }

        Ok(())
    }

    pub fn disassemble(
        code: &HashMap<usize, i64>,
        pointer: usize,
    ) -> Result<(String, usize), ComputerError> {
        let max = code.keys().max();
        let instruction = code.get(&pointer);

        if let Some((max, instruction)) = max.zip(instruction) {
            let width = ((16 - (max.leading_zeros() + 3) / 4) as usize).max(4);

            if let Ok((opcode, modes)) = AddrModes::analyze_instruction(*instruction) {
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
                            modes[num].format(p, width)
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
}
