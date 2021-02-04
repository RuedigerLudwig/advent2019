use std::{cell::Cell, fmt::Debug};

#[derive(Debug)]
pub enum Input {
    Value(i64),
    NoMoreValues,
    WaitForValue,
}

pub trait ComputerInput: Debug {
    fn get_next_input(&mut self) -> Input;
    fn provide_input(&mut self, value: i64);
}

#[derive(Debug, Clone)]
pub struct NoInput {}

impl ComputerInput for NoInput {
    fn get_next_input(&mut self) -> Input {
        Input::NoMoreValues
    }

    fn provide_input(&mut self, _value: i64) {}
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
    fn get_next_input(&mut self) -> Input {
        self._cell.take().map_or(Input::NoMoreValues, Input::Value)
    }
    fn provide_input(&mut self, _value: i64) {}
}
