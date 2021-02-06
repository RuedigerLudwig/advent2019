use crate::{
    cpu::{Cpu, CpuWrapper, MTCpuWrapper, STCpuWrapper},
    input::ComputerInput,
    Code, ComputerError, StepResult,
};

pub type STVirtualMachine<'a> = VirtualMachine<STCpuWrapper<'a>>;
pub type MTVirtualMachine<'a> = VirtualMachine<MTCpuWrapper<'a>>;

#[derive(Debug)]
pub struct VirtualMachine<W>
where
    W: CpuWrapper,
{
    _cpu_wrapper: W,
}

impl<'a> VirtualMachine<STCpuWrapper<'a>> {
    pub fn new_single(
        code: Code,
        input: impl ComputerInput + 'a,
    ) -> VirtualMachine<STCpuWrapper<'a>> {
        let cpu = STCpuWrapper::new(Cpu::new(code, input));
        VirtualMachine { _cpu_wrapper: cpu }
    }
}

impl<'a> VirtualMachine<MTCpuWrapper<'a>> {
    pub fn new_multi(
        code: Code,
        input: impl ComputerInput + 'a,
        id: usize,
    ) -> VirtualMachine<MTCpuWrapper<'a>> {
        let mut cpu = Cpu::new(code, input);
        cpu.set_id(id);
        let cpu = MTCpuWrapper::new(cpu);
        VirtualMachine { _cpu_wrapper: cpu }
    }
}

impl<'a, W> VirtualMachine<W>
where
    W: CpuWrapper,
{
    pub fn restart(&self) {
        self._cpu_wrapper.restart()
    }

    pub fn set_debug_level(&self, debug_level: u8) {
        self._cpu_wrapper.set_debug_level(debug_level);
    }

    pub fn patch_memory(&self, addr: usize, value: i64) {
        self._cpu_wrapper.patch_memory(addr, value);
    }

    pub fn get_memory(&self) -> Vec<i64> {
        self._cpu_wrapper.get_memory()
    }

    pub fn step(&self) -> Result<StepResult, ComputerError> {
        self._cpu_wrapper.step()
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
        self._cpu_wrapper.next()
    }
}
