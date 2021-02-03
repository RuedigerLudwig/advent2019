use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::ComputerInput;

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

    pub fn single(value: i64) -> Self {
        let list_input = Self::new();
        list_input.provide_input(value);
        list_input
    }

    pub fn provide_input(&self, input: i64) {
        self._list.borrow_mut().push_back(input);
    }
}

impl ComputerInput for ListInput {
    fn get_next_input(&self) -> Option<i64> {
        self._list.borrow_mut().pop_front()
    }
}
