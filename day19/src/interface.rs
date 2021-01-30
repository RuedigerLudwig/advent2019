#![allow(dead_code)]
use computer::{Code, ListInput, VirtualMachine};
use std::error::Error;

use common::Pos as RawPos;

use crate::tractor_error::TractorError;

pub type Pos = RawPos<i32>;

pub trait TractorInterface {
    fn check_pull(&mut self, position: Pos) -> Result<bool, Box<dyn Error>>;
}

pub struct TractorComputerInterface<'a> {
    _vm: VirtualMachine<'a, ListInput>,
}

impl<'a> TractorComputerInterface<'a> {
    pub fn new(code: &'a Code) -> TractorComputerInterface {
        TractorComputerInterface {
            _vm: VirtualMachine::new(code, &ListInput::new_()),
        }
    }
}

impl TractorInterface for TractorComputerInterface<'_> {
    fn check_pull(&mut self, position: Pos) -> Result<bool, Box<dyn Error>> {
        self._vm.restart();
        self._vm.get_input().provide_input(position.x() as i64);
        self._vm.get_input().provide_input(position.y() as i64);

        if let Some(result) = self._vm.get_output().next() {
            Ok(result? != 0)
        } else {
            Err(TractorError::NoData)?
        }
    }
}
