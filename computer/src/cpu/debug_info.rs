use common::join;

use crate::ComputerError;

use super::{
    common::disassemble, debug_codes, modes::AddrModes, operation_result::OperationResult, Cpu,
};

pub struct DebugInfo<'a> {
    cpu: &'a Cpu<'a>,
    id: &'a String,
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
            println!(
                "({}-{}) | {{{}}} | \"{}\"",
                self.range.0,
                self.range.1 - 1,
                self.offset,
                self.id
            );
        }

        if self.cpu.get_debug_level() & debug_codes::DISASSEMBLE != 0 {
            let (output, _) = disassemble(self.cpu.get_memory(), self.range.0)?;
            println!("    {}", output);
        }

        if self.cpu.get_debug_level() & debug_codes::FULL_INFO != 0 {
            println!("    Info: {:3}", self.name);
            println!(
                "          Inst:    {}",
                self.modes
                    .get(0)
                    .format(self.cpu.get_direct_value(self.range.0)?, 4)
            );
            print!("          Raw:     ");
            for ad in self.range.0 + 1..self.range.1 {
                let mode = self.modes.get(ad - self.range.0);
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
}
