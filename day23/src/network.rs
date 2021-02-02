use computer::{Code, ComputerInput, Output, StepResult, VirtualMachine};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::network_error::NetworkError;

#[derive(Debug)]
pub struct NodeInputQueue {
    _queue: VecDeque<i64>,
    _sent_empty: bool,
}

impl NodeInputQueue {
    pub fn new(id: i64) -> NodeInputQueue {
        let mut queue = VecDeque::new();
        queue.push_back(id);
        queue.push_back(-1);
        NodeInputQueue {
            _queue: queue,
            _sent_empty: false,
        }
    }

    pub fn feed(&mut self, x: i64, y: i64) {
        self._queue.push_back(x);
        self._queue.push_back(y);
    }

    pub fn is_active(&self) -> bool {
        !self._queue.is_empty() || !self._sent_empty
    }

    pub fn get_data(&mut self) -> i64 {
        if let Some(value) = self._queue.pop_front() {
            self._sent_empty = false;
            value
        } else {
            self._sent_empty = true;
            -1
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeInput {
    _node: Rc<RefCell<NodeInputQueue>>,
}

impl NodeInput {
    pub fn new(id: i64) -> NodeInput {
        NodeInput {
            _node: Rc::new(RefCell::new(NodeInputQueue::new(id))),
        }
    }

    pub fn feed_node(&self, x: i64, y: i64) {
        (*self._node).borrow_mut().feed(x, y)
    }

    pub fn is_active(&self) -> bool {
        (*self._node).borrow().is_active()
    }
}

impl ComputerInput for NodeInput {
    fn get_next_input(&self) -> Option<i64> {
        Some((*self._node).borrow_mut().get_data())
    }
}

#[derive(Debug)]
enum State {
    Value(i64, i64, i64),
    Active,
    Idle,
}

#[derive(Debug)]
struct NodeVm<'a> {
    _vm: VirtualMachine<'a, NodeInput>,
    _output: Output<NodeInput>,

    _next_receiver: Option<i64>,
    _next_x: Option<i64>,
}

impl<'a> NodeVm<'a> {
    pub fn new(vm: VirtualMachine<'a, NodeInput>) -> NodeVm<'a> {
        let output = vm.get_output();
        NodeVm {
            _vm: vm,
            _output: output,
            _next_receiver: None,
            _next_x: None,
        }
    }

    pub fn step(&mut self) -> Result<State, NetworkError> {
        if self.is_active() {
            match self._output.step()? {
                StepResult::Value(value) => {
                    if self._next_receiver.is_none() {
                        self._next_receiver = Some(value);
                        Ok(State::Active)
                    } else if self._next_x.is_none() {
                        self._next_x = Some(value);
                        Ok(State::Active)
                    } else {
                        let result = State::Value(
                            self._next_receiver.unwrap(),
                            self._next_x.unwrap(),
                            value,
                        );
                        self._next_receiver = None;
                        self._next_x = None;
                        Ok(result)
                    }
                }
                StepResult::Proceed => Ok(State::Active),
                StepResult::Stop => Err(NetworkError::NodeStopped),
            }
        } else {
            Ok(State::Idle)
        }
    }

    pub fn is_active(&self) -> bool {
        self._next_receiver.is_some() || self._vm.get_input().is_active()
    }
}

#[derive(Debug)]
pub struct Switch<'a> {
    _vms: Vec<NodeVm<'a>>,
    _inputs: Vec<NodeInput>,
}

impl<'a> Switch<'a> {
    pub fn new(code: &Code, count: usize) -> Switch {
        let mut vms = Vec::with_capacity(count);
        let mut inputs = Vec::with_capacity(count);

        for number in 0..count {
            let input = NodeInput::new(number as i64);
            let node_vm = NodeVm::new(VirtualMachine::with_id(&code, &input, &number.to_string()));

            inputs.push(input);
            vms.push(node_vm);
        }

        Switch {
            _vms: vms,
            _inputs: inputs,
        }
    }

    pub fn part1(&mut self) -> Result<i64, NetworkError> {
        loop {
            for vm in self._vms.iter_mut() {
                if let State::Value(receiver, x, y) = vm.step()? {
                    match receiver {
                        0..=49 => self._inputs[receiver as usize].feed_node(x, y),
                        255 => return Ok(y),
                        _ => return Err(NetworkError::UnknownAddress(receiver)),
                    }
                }
            }
        }
    }

    pub fn part2(&mut self) -> Result<i64, NetworkError> {
        let mut nat_memory = None;
        let mut last_delivered = None;
        loop {
            let mut all_idle = true;
            for vm in self._vms.iter_mut() {
                match vm.step()? {
                    State::Value(receiver, x, y) => {
                        match receiver {
                            0..=49 => self._inputs[receiver as usize].feed_node(x, y),
                            255 => nat_memory = Some((x, y)),

                            _ => return Err(NetworkError::UnknownAddress(receiver)),
                        }
                        all_idle = false;
                    }
                    State::Active => all_idle = false,
                    State::Idle => {}
                }
            }

            if all_idle {
                if let Some((x, y)) = nat_memory {
                    if last_delivered.map_or(false, |old_y| old_y == y) {
                        return Ok(y);
                    }
                    self._inputs[0].feed_node(x, y);
                    last_delivered = Some(y);
                }
            }
        }
    }
}
