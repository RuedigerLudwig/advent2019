use std::cell::Cell;

#[derive(Debug)]
pub enum Input {
    Value(i64),
    NoMoreValues,
    WaitForValue,
}

pub trait ComputerInput: Clone {
    fn get_next_input(&self) -> Input;
}

#[derive(Debug, Clone)]
pub struct NoInput {}

impl ComputerInput for NoInput {
    fn get_next_input(&self) -> Input {
        Input::NoMoreValues
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
    fn get_next_input(&self) -> Input {
        self._cell.take().map_or(Input::NoMoreValues, Input::Value)
    }
}
