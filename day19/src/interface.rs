use crate::error::TractorError;
use computer::{Code, ListInput, STVirtualMachine};

use common::Pos as RawPos;

pub type Pos = RawPos<i32>;

pub trait TractorInterface {
    fn check_pull(&mut self, position: Pos) -> Result<bool, TractorError>;
}

pub struct TractorComputerInterface<'a> {
    vm: STVirtualMachine<'a>,
    input: ListInput,
}

impl<'a> TractorComputerInterface<'a> {
    pub fn new(code: Code) -> TractorComputerInterface<'a> {
        let input = ListInput::new();
        TractorComputerInterface {
            vm: STVirtualMachine::new_single(code, input.clone()),
            input,
        }
    }
}

impl TractorInterface for TractorComputerInterface<'_> {
    fn check_pull(&mut self, position: Pos) -> Result<bool, TractorError> {
        self.vm.restart();
        self.input.clear();
        self.input.provide_input(position.x() as i64);
        self.input.provide_input(position.y() as i64);

        self.vm
            .next()?
            .map(|result| result != 0)
            .ok_or(TractorError::NoData)
    }
}
