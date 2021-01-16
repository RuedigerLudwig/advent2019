use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub trait ComputerInput: Clone {
    fn provide_input(&self, input: i64);
    fn get_next_input(&self) -> Option<i64>;
}

#[derive(Debug, Clone)]
pub struct ListInput {
    _list: Rc<RefCell<VecDeque<i64>>>,
}

impl ListInput {
    pub fn new() -> ListInput {
        ListInput {
            _list: Rc::new(RefCell::new(VecDeque::new())),
        }
    }
}

impl ComputerInput for ListInput {
    fn provide_input(&self, input: i64) {
        self._list.borrow_mut().push_back(input);
    }

    fn get_next_input(&self) -> Option<i64> {
        self._list.borrow_mut().pop_front()
    }
}
