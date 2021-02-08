use super::error::TractorError;
use crate::common::pos::Pos as RawPos;
use crate::computer::{Code, ListInput, VirtualMachine};

pub type Pos = RawPos<i64>;

pub trait TractorInterface {
    fn check_pull(&mut self, position: Pos) -> Result<bool, TractorError>;
}

pub struct TractorComputerInterface<'a> {
    vm: VirtualMachine<'a>,
    input: ListInput,
}

impl<'a> TractorComputerInterface<'a> {
    pub fn new(code: Code) -> TractorComputerInterface<'a> {
        let input = ListInput::new();
        TractorComputerInterface {
            vm: VirtualMachine::new(code, input.clone()),
            input,
        }
    }
}

impl TractorInterface for TractorComputerInterface<'_> {
    fn check_pull(&mut self, position: Pos) -> Result<bool, TractorError> {
        self.vm.restart();
        self.input.clear();
        self.input.provide(position)?;

        self.vm
            .next()?
            .map(|result| result != 0)
            .ok_or(TractorError::NoData)
    }
}
