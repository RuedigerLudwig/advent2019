use std::{cell::RefCell, rc::Rc};

use crate::{cpu::Cpu, input::ComputerInput, output::Output, Code, ComputerError, ListInput};

pub struct VirtualMachine<'a, I>
where
    I: ComputerInput,
{
    _input: I,
    _code: &'a Code,
    _cpu: CpuWrapper<I>,
}

impl<'a> VirtualMachine<'a, ListInput> {
    pub fn new(code: &Code) -> VirtualMachine<ListInput> {
        let input = ListInput::new();
        let cpu = CpuWrapper::new(Cpu::new(code.get().clone(), input.clone()));
        VirtualMachine {
            _code: code,
            _input: input,
            _cpu: cpu,
        }
    }
}

impl<'a, I> VirtualMachine<'a, I>
where
    I: ComputerInput,
{
    pub fn with_input(code: &'a Code, input: I) -> VirtualMachine<I> {
        let cpu = CpuWrapper::new(Cpu::new(code.get().clone(), input.clone()));
        VirtualMachine {
            _code: code,
            _input: input,
            _cpu: cpu,
        }
    }

    pub fn patch_memory(&self, addr: usize, value: i64) {
        self._cpu.patch_memory(addr, value)
    }

    pub fn get_memory(&self) -> Vec<i64> {
        self._cpu.get_memory()
    }

    pub fn get_output(&self) -> Output<I> {
        Output::new(self._cpu.clone())
    }

    pub fn get_input(&self) -> I {
        self._input.clone()
    }

    pub fn restart(&mut self) {
        self._cpu = CpuWrapper::new(Cpu::new(self._code.get().clone(), self._input.clone()));
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

    pub fn patch_memory(&self, addr: usize, value: i64) {
        (*self._cpu.borrow_mut()).patch_memory(addr, value)
    }

    pub fn get_memory(&self) -> Vec<i64> {
        (*self._cpu.borrow()).get_memory()
    }

    pub fn step(&self) -> Result<Option<i64>, ComputerError> {
        (*self._cpu.borrow_mut()).step()
    }
}
