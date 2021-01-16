use std::{cell::RefCell, rc::Rc};

use crate::{cpu::Cpu, input::ComputerInput, output::Output, Code, ComputerError, ListInput};

pub struct VirtualMachine<I>
where
    I: ComputerInput,
{
    _input: I,
    _cpu: CpuWrapper<I>,
}

impl VirtualMachine<ListInput> {
    pub fn new(code: &Code) -> VirtualMachine<ListInput> {
        let input = ListInput::new();
        let cpu = CpuWrapper::new(Cpu::create(code.get(), input.clone()));
        VirtualMachine {
            _input: input,
            _cpu: cpu,
        }
    }
}

impl<I> VirtualMachine<I>
where
    I: ComputerInput,
{
    pub fn with_input(code: &Code, input: I) -> VirtualMachine<I> {
        let cpu = CpuWrapper::new(Cpu::create(code.get(), input.clone()));
        VirtualMachine {
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
