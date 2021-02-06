use crate::{
    cpu::{Cpu, CpuWrapper, MTCpuWrapper, STCpuWrapper},
    input::ComputerInput,
    output::RawOutput,
    Code,
};

pub type STVirtualMachine<'a> = VirtualMachine<STCpuWrapper<'a>>;
pub type MTVirtualMachine<'a> = VirtualMachine<MTCpuWrapper<'a>>;

#[derive(Debug)]
pub struct VirtualMachine<W>
where
    W: CpuWrapper,
{
    _cpu: W,
}

impl<'a> VirtualMachine<STCpuWrapper<'a>> {
    pub fn new(code: Code, input: impl ComputerInput + 'a) -> VirtualMachine<STCpuWrapper<'a>> {
        let cpu = STCpuWrapper::new(Cpu::new(code, input));
        VirtualMachine { _cpu: cpu }
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
        VirtualMachine { _cpu: cpu }
    }
}

impl<'a, W> VirtualMachine<W>
where
    W: CpuWrapper,
{
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

    pub fn get_output(&self) -> RawOutput<W> {
        RawOutput::new(self._cpu.clone())
    }

    pub fn provide_input(&self, value: i64) {
        self._cpu.provide_input(value)
    }
}
