use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::ComputerInput;

#[derive(Debug, Clone)]
pub struct ListInput {
    _list: Rc<RefCell<VecDeque<i64>>>,
}

impl ListInput {
    pub fn new() -> Self {
        Self {
            _list: Rc::default(),
        }
    }

    pub fn provide_input(&mut self, input: i64) {
        (*self._list.borrow_mut()).push_back(input)
    }

    pub fn clear(&mut self) {
        (*self._list.borrow_mut()).clear()
    }
}

impl ComputerInput for ListInput {
    fn get_next_input(&mut self) -> Option<i64> {
        (*self._list.borrow_mut()).pop_front()
    }
}
