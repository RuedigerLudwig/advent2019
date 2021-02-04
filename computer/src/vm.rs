use std::{cell::RefCell, rc::Rc};

use crate::{
    cpu::{Cpu, StepResult},
    input::ComputerInput,
    output::Output,
    Code, ComputerError,
};

#[derive(Debug)]
pub struct VirtualMachine<'a> {
    _cpu: CpuWrapper<'a>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(code: &'a Code, input: impl ComputerInput + 'a) -> VirtualMachine<'a> {
        let cpu = CpuWrapper::new(Cpu::new(code, input));
        VirtualMachine { _cpu: cpu }
    }

    pub fn with_id(code: &'a Code, input: impl ComputerInput + 'a, id: &str) -> VirtualMachine<'a> {
        let mut cpu = Cpu::new(code, input);
        cpu.set_id(id);
        let cpu = CpuWrapper::new(cpu);
        VirtualMachine { _cpu: cpu }
    }

    pub fn restart(&self) {
        self._cpu.restart()
    }

    pub fn set_debug_level(&self, debug_level: u8) {
        self._cpu.set_debug_level(debug_level);
    }

    pub fn patch_memory(&self, addr: usize, value: i64) {
        self._cpu.patch_memory(addr, value);
    }

    pub fn get_memory(&self) -> Vec<i64> {
        self._cpu.get_memory()
    }

    pub fn get_output(&self) -> Output<'a> {
        Output::new(self._cpu.clone())
    }

    pub fn provide_input(&self, value: i64) {
        self._cpu.provide_input(value)
    }
}

#[derive(Debug, Clone)]
pub struct CpuWrapper<'a> {
    _cpu: Rc<RefCell<Cpu<'a>>>,
}

impl<'a> CpuWrapper<'a> {
    pub fn new(cpu: Cpu<'a>) -> CpuWrapper<'a> {
        CpuWrapper {
            _cpu: Rc::new(RefCell::new(cpu)),
        }
    }

    pub fn restart(&self) {
        (*self._cpu.borrow_mut()).restart();
    }

    pub fn set_debug_level(&self, debug_level: u8) {
        (*self._cpu.borrow_mut()).set_debug_level(debug_level);
    }

    pub fn patch_memory(&self, addr: usize, value: i64) {
        (*self._cpu.borrow_mut()).patch_memory(addr, value)
    }

    pub fn get_memory(&self) -> Vec<i64> {
        (*self._cpu.borrow()).get_memory()
    }

    pub fn provide_input(&self, value: i64) {
        (*self._cpu.borrow_mut()).provide_input(value)
    }

    pub fn step(&self) -> Result<StepResult, ComputerError> {
        (*self._cpu.borrow_mut()).step()
    }
}
