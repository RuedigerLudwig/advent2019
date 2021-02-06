use crate::error::TractorError;
use common::pos::Pos as RawPos;
use computer::{Code, ListInput, VirtualMachine};

pub type Pos = RawPos<i32>;

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
        self.input.provide_input(position.x() as i64);
        self.input.provide_input(position.y() as i64);

        self.vm
            .next()?
            .map(|result| result != 0)
            .ok_or(TractorError::NoData)
    }
}
