use std::cell::Cell;

use crate::{
    cpu::{CpuWrapper, MTCpuWrapper, STCpuWrapper},
    vm::VirtualMachine,
    Code, ComputerError, ComputerInput, StepResult,
};

pub type STTextVM<'a> = TextVM<STCpuWrapper<'a>>;
pub type MTTextVM<'a> = TextVM<MTCpuWrapper<'a>>;

impl<'a> TextVM<STCpuWrapper<'a>> {
    pub fn new_single(code: Code, input: impl ComputerInput + 'a) -> TextVM<STCpuWrapper<'a>> {
        TextVM::new(VirtualMachine::new_single(code, input))
    }
}

impl<'a> TextVM<MTCpuWrapper<'a>> {
    pub fn new_multi(
        code: Code,
        input: impl ComputerInput + 'a,
        id: usize,
    ) -> TextVM<MTCpuWrapper<'a>> {
        TextVM::new(VirtualMachine::new_multi(code, input, id))
    }
}

#[derive(Debug)]
pub struct TextVM<W>
where
    W: CpuWrapper,
{
    _vm: VirtualMachine<W>,
    _peek: Cell<Option<Option<i64>>>,
}

impl<'a, W> TextVM<W>
where
    W: CpuWrapper,
{
    pub fn new(vm: VirtualMachine<W>) -> TextVM<W> {
        TextVM {
            _vm: vm,
            _peek: Cell::default(),
        }
    }

    pub fn read_line(&self) -> Result<Option<String>, ComputerError> {
        if let Some(peeked) = self.peek()? {
            match peeked {
                0..=127 => (),
                _ => return Ok(None),
            }
        }

        let mut result = String::new();
        while let Some(item) = self.next()? {
            match item {
                10 => return Ok(Some(result)),
                n @ 0..=127 => result.push((n as u8) as char),
                n => return Err(ComputerError::NotValidAsciiInt(n)),
            }
        }
        Ok(None)
    }

    pub fn restart(&self) {
        self._vm.restart()
    }

    pub fn set_debug_level(&self, debug_level: u8) {
        self._vm.set_debug_level(debug_level)
    }

    pub fn patch_memory(&self, addr: usize, value: i64) {
        self._vm.patch_memory(addr, value)
    }

    pub fn get_memory(&self) -> Vec<i64> {
        self._vm.get_memory()
    }

    pub fn step(&self) -> Result<StepResult, ComputerError> {
        self._peek.set(None);
        self._vm.step()
    }

    pub fn get_all(&self) -> Result<Vec<i64>, ComputerError> {
        let mut result = Vec::new();
        while let Some(compute) = self.next()? {
            result.push(compute);
        }
        Ok(result)
    }

    pub fn take_exactly(&self, count: usize) -> Result<Option<Vec<i64>>, ComputerError> {
        let mut result = Vec::new();

        for _ in 0..count {
            if let Some(compute) = self.next()? {
                result.push(compute)
            } else {
                return Ok(None);
            }
        }

        Ok(Some(result))
    }

    pub fn next(&self) -> Result<Option<i64>, ComputerError> {
        if let Some(peek) = self._peek.take() {
            return Ok(peek);
        }
        self._vm.next()
    }

    pub fn peek(&self) -> Result<Option<i64>, ComputerError> {
        if let Some(peek) = self._peek.get() {
            Ok(peek)
        } else {
            let peek = self.next()?;
            self._peek.set(Some(peek));
            Ok(peek)
        }
    }
}
