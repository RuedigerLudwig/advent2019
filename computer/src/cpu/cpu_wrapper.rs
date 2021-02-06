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
    fn step(&self) -> Result<StepResult, ComputerError>;
    fn next(&self) -> Result<Option<i64>, ComputerError>;
}

#[derive(Debug)]
pub struct STCpuWrapper<'a> {
    _cpu: Rc<RefCell<Cpu<'a>>>,
}

impl Clone for STCpuWrapper<'_> {
    fn clone(&self) -> Self {
        Self {
            _cpu: Rc::clone(&self._cpu),
        }
    }
}

impl<'a> STCpuWrapper<'a> {
    pub fn new(cpu: Cpu<'a>) -> STCpuWrapper<'a> {
        STCpuWrapper {
            _cpu: Rc::new(RefCell::new(cpu)),
        }
    }
}

impl<'a> CpuWrapper for STCpuWrapper<'a> {
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
        (*self._cpu.borrow()).get_linear_memory()
    }

    fn step(&self) -> Result<StepResult, ComputerError> {
        (*self._cpu.borrow_mut()).step()
    }

    fn next(&self) -> Result<Option<i64>, ComputerError> {
        let mut cpu = self._cpu.borrow_mut();
        loop {
            match cpu.step()? {
                StepResult::Value(value) => return Ok(Some(value)),
                StepResult::Stop => return Ok(None),
                StepResult::Proceed => (),
                StepResult::WaitForInput => return Err(ComputerError::InputEmpty),
            }
        }
    }
}

#[derive(Debug)]
pub struct MTCpuWrapper<'a> {
    _cpu: Arc<Mutex<Cpu<'a>>>,
}

impl Clone for MTCpuWrapper<'_> {
    fn clone(&self) -> Self {
        Self {
            _cpu: Arc::clone(&self._cpu),
        }
    }
}

impl<'a> MTCpuWrapper<'a> {
    pub fn new(cpu: Cpu<'a>) -> MTCpuWrapper<'a> {
        MTCpuWrapper {
            _cpu: Arc::new(Mutex::new(cpu)),
        }
    }
}

impl<'a> CpuWrapper for MTCpuWrapper<'a> {
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
        cpu.get_linear_memory()
    }

    fn step(&self) -> Result<StepResult, ComputerError> {
        let mut cpu = self._cpu.lock().unwrap();
        cpu.step()
    }

    fn next(&self) -> Result<Option<i64>, ComputerError> {
        loop {
            match self.step()? {
                StepResult::Value(value) => return Ok(Some(value)),
                StepResult::Stop => return Ok(None),
                StepResult::Proceed => (),
                StepResult::WaitForInput => (), // TODO!!
            }
        }
    }
}
