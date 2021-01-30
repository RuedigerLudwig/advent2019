pub trait ComputerInput: Clone {
    fn get_next_input(&self) -> Option<i64>;
}

#[derive(Debug, Clone)]
pub struct NoInput {}

impl ComputerInput for NoInput {
    fn get_next_input(&self) -> Option<i64> {
        unimplemented!("Never call Iput on a NoInput")
    }
}
