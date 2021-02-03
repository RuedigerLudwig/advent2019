use std::{cell::RefCell, rc::Rc};

use crate::{
    cpu::{Cpu, StepResult},
    input::ComputerInput,
    output::Output,
    Code, ComputerError,
};

#[derive(Debug)]
pub struct VirtualMachine<'a, I>
where
    I: ComputerInput,
{
    _input: I,
    _code: &'a Code,
    _cpu: CpuWrapper<I>,
}

impl<'a, I> VirtualMachine<'a, I>
where
    I: ComputerInput,
{
    pub fn new(code: &'a Code, input: &'_ I) -> VirtualMachine<'a, I> {
        let cpu = CpuWrapper::new(Cpu::new(code.as_ref().clone(), input.clone()));
        VirtualMachine {
            _code: code,
            _input: input.clone(),
            _cpu: cpu,
        }
    }

    pub fn with_id(code: &'a Code, input: &'_ I, id: &str) -> VirtualMachine<'a, I> {
        let mut cpu = Cpu::new(code.as_ref().clone(), input.clone());
        cpu.set_id(id);
        let cpu = CpuWrapper::new(cpu);
        VirtualMachine {
            _code: code,
            _input: input.clone(),
            _cpu: cpu,
        }
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

    pub fn get_input(&self) -> I {
        self._input.clone()
    }

    pub fn get_output(&self) -> Output<I> {
        Output::new(self._cpu.clone())
    }

    pub fn restart(&mut self) {
        self._cpu = CpuWrapper::new(Cpu::new(self._code.as_ref().clone(), self._input.clone()));
    }
}

#[derive(Debug, Clone)]
pub struct CpuWrapper<I> {
    _cpu: Rc<RefCell<Cpu<I>>>,
}

impl<'a, I> CpuWrapper<I>
where
    I: ComputerInput,
{
    pub fn new(cpu: Cpu<I>) -> CpuWrapper<I> {
        CpuWrapper {
            _cpu: Rc::new(RefCell::new(cpu)),
        }
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

    pub fn step(&self) -> Result<StepResult, ComputerError> {
        (*self._cpu.borrow_mut()).step()
    }
}
