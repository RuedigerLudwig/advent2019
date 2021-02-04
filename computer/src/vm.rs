use crate::{
    cpu::Cpu,
    cpu_wrapper::{CpuWrapper, MultiThreadWrapper, SingleThreadWrapper},
    input::ComputerInput,
    output::RawOutput,
    Code,
};

pub type STVirtualMachine<'a> = VirtualMachine<SingleThreadWrapper<'a>>;
pub type MTVirtualMachine<'a> = VirtualMachine<MultiThreadWrapper<'a>>;

#[derive(Debug)]
pub struct VirtualMachine<W>
where
    W: CpuWrapper,
{
    _cpu: W,
}

impl<'a> VirtualMachine<SingleThreadWrapper<'a>> {
    pub fn new(
        code: &'a Code,
        input: impl ComputerInput + 'a,
    ) -> VirtualMachine<SingleThreadWrapper<'a>> {
        let cpu = SingleThreadWrapper::new(Cpu::new(code, input));
        VirtualMachine { _cpu: cpu }
    }
}

impl<'a> VirtualMachine<MultiThreadWrapper<'a>> {
    pub fn new_multi(
        code: &'a Code,
        input: impl ComputerInput + 'a,
        id: &str,
    ) -> VirtualMachine<MultiThreadWrapper<'a>> {
        let mut cpu = Cpu::new(code, input);
        cpu.set_id(id);
        let cpu = MultiThreadWrapper::new(cpu);
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
