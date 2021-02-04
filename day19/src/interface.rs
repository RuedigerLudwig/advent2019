use crate::error::TractorError;
use computer::{Code, ListInput, STVirtualMachine};

use common::Pos as RawPos;

pub type Pos = RawPos<i32>;

pub trait TractorInterface {
    fn check_pull(&mut self, position: Pos) -> Result<bool, TractorError>;
}

pub struct TractorComputerInterface<'a> {
    _vm: STVirtualMachine<'a>,
}

impl<'a> TractorComputerInterface<'a> {
    pub fn new(code: &'a Code) -> TractorComputerInterface<'_> {
        TractorComputerInterface {
            _vm: STVirtualMachine::new(code, ListInput::new()),
        }
    }
}

impl TractorInterface for TractorComputerInterface<'_> {
    fn check_pull(&mut self, position: Pos) -> Result<bool, TractorError> {
        self._vm.restart();
        self._vm.provide_input(position.x() as i64);
        self._vm.provide_input(position.y() as i64);

        self._vm
            .get_output()
            .next()?
            .map(|result| result != 0)
            .ok_or(TractorError::NoData)
    }
}
