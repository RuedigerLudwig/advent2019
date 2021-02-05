use std::collections::VecDeque;

use crate::ComputerInput;

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
    fn get_next_input(&mut self) -> Option<i64> {
        self._list.pop_front()
    }

    fn provide_input(&mut self, input: i64) {
        self._list.push_back(input);
    }
}
