use crate::{ComputerError, STVirtualMachine};

pub trait InputConverter {
    fn send_to(self, vm: &STVirtualMachine<'_>) -> Result<(), ComputerError>;
}

impl InputConverter for String {
    fn send_to(self, vm: &STVirtualMachine<'_>) -> Result<(), ComputerError> {
        self.as_str().send_to(vm)
    }
}

impl InputConverter for &String {
    fn send_to(self, vm: &STVirtualMachine<'_>) -> Result<(), ComputerError> {
        self.as_str().send_to(vm)
    }
}

impl InputConverter for &str {
    fn send_to(self, vm: &STVirtualMachine<'_>) -> Result<(), ComputerError> {
        for ch in self.chars() {
            let val = ch as i64;
            if 0 <= val && val <= 127 {
                vm.provide_input(val);
            } else {
                return Err(ComputerError::NotValidAsciiChar(ch));
            }
        }
        vm.provide_input(10);

        Ok(())
    }
}
