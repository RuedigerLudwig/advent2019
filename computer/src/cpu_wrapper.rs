use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::{cpu::Cpu, ComputerError, StepResult};

pub trait CpuWrapper: Clone {
    fn restart(&self);
    fn set_debug_level(&self, debug_level: u8);
    fn patch_memory(&self, addr: usize, value: i64);
    fn get_memory(&self) -> Vec<i64>;
    fn provide_input(&self, value: i64);
    fn step(&self) -> Result<StepResult, ComputerError>;
}

#[derive(Debug, Clone)]
pub struct SingleThreadWrapper<'a> {
    _cpu: Rc<RefCell<Cpu<'a>>>,
}

impl<'a> SingleThreadWrapper<'a> {
    pub fn new(cpu: Cpu<'a>) -> SingleThreadWrapper<'a> {
        SingleThreadWrapper {
            _cpu: Rc::new(RefCell::new(cpu)),
        }
    }
}

impl<'a> CpuWrapper for SingleThreadWrapper<'a> {
    fn restart(&self) {
        (*self._cpu.borrow_mut()).restart();
    }

    fn set_debug_level(&self, debug_level: u8) {
        (*self._cpu.borrow_mut()).set_debug_level(debug_level);
    }

    fn patch_memory(&self, addr: usize, value: i64) {
        (*self._cpu.borrow_mut()).patch_memory(addr, value)
    }

    fn get_memory(&self) -> Vec<i64> {
        (*self._cpu.borrow()).get_memory()
    }

    fn provide_input(&self, value: i64) {
        (*self._cpu.borrow_mut()).provide_input(value)
    }

    fn step(&self) -> Result<StepResult, ComputerError> {
        (*self._cpu.borrow_mut()).step()
    }
}

#[derive(Debug, Clone)]
pub struct MultiThreadWrapper<'a> {
    _cpu: Arc<Mutex<Cpu<'a>>>,
}

impl<'a> MultiThreadWrapper<'a> {
    pub fn new(cpu: Cpu<'a>) -> MultiThreadWrapper<'a> {
        MultiThreadWrapper {
            _cpu: Arc::new(Mutex::new(cpu)),
        }
    }
}

impl<'a> CpuWrapper for MultiThreadWrapper<'a> {
    fn restart(&self) {
        let mut cpu = self._cpu.lock().unwrap();
        cpu.restart();
    }

    fn set_debug_level(&self, debug_level: u8) {
        let mut cpu = self._cpu.lock().unwrap();
        cpu.set_debug_level(debug_level);
    }

    fn patch_memory(&self, addr: usize, value: i64) {
        let mut cpu = self._cpu.lock().unwrap();
        cpu.patch_memory(addr, value)
    }

    fn get_memory(&self) -> Vec<i64> {
        let cpu = self._cpu.lock().unwrap();
        cpu.get_memory()
    }

    fn provide_input(&self, value: i64) {
        let mut cpu = self._cpu.lock().unwrap();
        cpu.provide_input(value)
    }

    fn step(&self) -> Result<StepResult, ComputerError> {
        let mut cpu = self._cpu.lock().unwrap();
        cpu.step()
    }
}
