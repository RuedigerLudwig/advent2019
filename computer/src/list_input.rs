use crate::ComputerInput;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug, Clone)]
pub struct ListInput {
    list: Rc<RefCell<VecDeque<i64>>>,
}

impl ListInput {
    pub fn new() -> Self {
        Self {
            list: Rc::default(),
        }
    }

    pub fn provide_input(&mut self, input: i64) {
        (*self.list.borrow_mut()).push_back(input)
    }

    pub fn clear(&mut self) {
        (*self.list.borrow_mut()).clear()
    }
}

impl ComputerInput for ListInput {
    fn get_next_input(&mut self) -> Option<i64> {
        (*self.list.borrow_mut()).pop_front()
    }
}
