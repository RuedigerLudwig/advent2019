use std::fmt::Debug;

pub trait ComputerInput: Debug {
    fn get_next_input(&mut self) -> Option<i64>;
}

#[derive(Debug)]
pub struct NoInput {}

impl ComputerInput for NoInput {
    fn get_next_input(&mut self) -> Option<i64> {
        None
    }
}

#[derive(Debug)]
pub struct OnceInput {
    cell: Option<i64>,
}

impl OnceInput {
    pub fn new(value: i64) -> OnceInput {
        OnceInput { cell: Some(value) }
    }
}

impl ComputerInput for OnceInput {
    fn get_next_input(&mut self) -> Option<i64> {
        self.cell.take()
    }
}
