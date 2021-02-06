use std::{cell::Cell, fmt::Debug};

pub trait ComputerInput: Debug {
    fn get_next_input(&mut self) -> Option<i64>;
    //fn provide_input(&mut self, value: i64);
}

#[derive(Debug, Clone)]
pub struct NoInput {}

impl ComputerInput for NoInput {
    fn get_next_input(&mut self) -> Option<i64> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct OnceInput {
    _cell: Cell<Option<i64>>,
}

impl OnceInput {
    pub fn new(value: i64) -> OnceInput {
        OnceInput {
            _cell: Cell::new(Some(value)),
        }
    }
}

impl ComputerInput for OnceInput {
    fn get_next_input(&mut self) -> Option<i64> {
        self._cell.take()
    }
}
