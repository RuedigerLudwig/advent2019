use std::collections::VecDeque;

pub trait ComputerInput {
    fn provide_input(&mut self, input: i64);
    fn get_next_input(&mut self) -> Option<i64>;
}

#[derive(Debug, Clone, Default)]
pub struct ListInput {
    _list: VecDeque<i64>,
}

impl ComputerInput for ListInput {
    fn provide_input(&mut self, input: i64) {
        self._list.push_back(input);
    }

    fn get_next_input(&mut self) -> Option<i64> {
        self._list.pop_front()
    }
}
