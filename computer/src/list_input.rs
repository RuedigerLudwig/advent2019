use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{input::Input, ComputerInput};

#[derive(Debug, Clone)]
pub struct ListInput {
    _list: Rc<RefCell<VecDeque<i64>>>,
}

impl ListInput {
    pub fn new() -> Self {
        Self {
            _list: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    pub fn provide_input(&self, input: i64) {
        self._list.borrow_mut().push_back(input);
    }
}

impl ComputerInput for ListInput {
    fn get_next_input(&self) -> Input {
        self._list
            .borrow_mut()
            .pop_front()
            .map_or(Input::NoMoreValues, Input::Value)
    }
}
