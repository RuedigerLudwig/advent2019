use std::collections::VecDeque;

use crate::{input::Input, ComputerInput};

#[derive(Debug, Clone)]
pub struct ListInput {
    _list: VecDeque<i64>,
}

impl ListInput {
    pub fn new() -> Self {
        Self {
            _list: VecDeque::new(),
        }
    }
}

impl ComputerInput for ListInput {
    fn get_next_input(&mut self) -> Input {
        self._list
            .pop_front()
            .map_or(Input::NoMoreValues, Input::Value)
    }

    fn provide_input(&mut self, input: i64) {
        self._list.push_back(input);
    }
}
