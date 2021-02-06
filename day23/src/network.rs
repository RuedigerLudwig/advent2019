use crate::error::NetworkError;
use computer::{Code, ComputerInput, MTVirtualMachine};
use mpsc::{channel, TryRecvError};
use std::{
    collections::{HashSet, VecDeque},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Condvar, Mutex,
    },
    thread,
};

#[derive(Debug)]
enum NodeState {
    Active,
    Inactive,
    Terminated,
}

#[derive(Debug)]
struct NodeData {
    queue: VecDeque<i64>,
    from_queue: bool,
    terminated: bool,
}

#[derive(Debug, Clone)]
struct Node {
    _id: usize,
    _data: Arc<(Mutex<NodeData>, Condvar)>,
}

impl Node {
    pub fn new(id: usize) -> Node {
        let mut queue = VecDeque::new();
        queue.push_back(id as i64);
        queue.push_back(-1);
        Node {
            _id: id,
            _data: Arc::new((
                Mutex::new(NodeData {
                    queue,
                    from_queue: true,
                    terminated: false,
                }),
                Condvar::new(),
            )),
        }
    }

    pub fn get_id(&self) -> usize {
        return self._id;
    }

    pub fn feed(&self, x: i64, y: i64) {
        let (guard, wake_upper) = &*self._data;
        let mut data = guard.lock().unwrap();
        data.queue.push_back(x);
        data.queue.push_back(y);
        wake_upper.notify_one();
    }

    pub fn terminate(&self) {
        let (guard, wake_upper) = &*self._data;
        let mut data = guard.lock().unwrap();
        data.terminated = true;
        wake_upper.notify_one();
    }

    pub fn get_state(&self) -> NodeState {
        let (guard, _) = &*self._data;
        let data = guard.lock().unwrap();
        if data.terminated {
            NodeState::Terminated
        } else if !data.queue.is_empty() || data.from_queue {
            NodeState::Active
        } else {
            NodeState::Inactive
        }
    }

    pub fn wait_for_signal(&self) {
        let (guard, wake_upper) = &*self._data;
        let mut data = guard.lock().unwrap();
        while data.queue.is_empty() && !data.terminated {
            data = wake_upper.wait(data).unwrap();
        }
    }
}

impl ComputerInput for Node {
    fn get_next_input(&mut self) -> Option<i64> {
        let (guard, _) = &*self._data;
        let mut data = guard.lock().unwrap();
        if let Some(value) = data.queue.pop_front() {
            data.from_queue = true;
            Some(value)
        } else {
            data.from_queue = false;
            Some(-1)
        }
    }

    fn provide_input(&mut self, _value: i64) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum ThreadResult {
    Result {
        from: usize,
        to: usize,
        x: i64,
        y: i64,
    },
    Inactive {
        from: usize,
    },
    Active {
        from: usize,
    },
}

#[derive(Debug)]
struct NodeVm<'a> {
    _vm: MTVirtualMachine<'a>,
    _node: Node,

    _result_tx: Sender<ThreadResult>,

    _next_receiver: Option<usize>,
    _next_x: Option<i64>,
}

impl<'a> NodeVm<'a> {
    pub fn new(code: Code, node: Node, result_tx: Sender<ThreadResult>) -> NodeVm<'a> {
        let vm = MTVirtualMachine::new_multi(code, node.clone(), node.get_id());

        NodeVm {
            _vm: vm,
            _node: node,

            _result_tx: result_tx,

            _next_receiver: None,
            _next_x: None,
        }
    }

    pub fn run(&mut self) -> Result<(), NetworkError> {
        let output = self._vm.get_output();
        loop {
            let active = match output.step()? {
                computer::StepResult::Value(value) => {
                    if self._next_receiver.is_none() {
                        self._next_receiver = Some(value as usize);
                    } else if self._next_x.is_none() {
                        self._next_x = Some(value);
                    } else {
                        let result = ThreadResult::Result {
                            from: self._node._id,
                            to: self._next_receiver.unwrap(),
                            x: self._next_x.unwrap(),
                            y: value,
                        };
                        self._next_receiver = None;
                        self._next_x = None;
                        self._result_tx.send(result)?;
                    };
                    true
                }
                computer::StepResult::Stop => return Err(NetworkError::NodeStopped),
                computer::StepResult::Proceed => false,
                computer::StepResult::WaitForInput => false,
            };

            let mut state = self._node.get_state();
            match state {
                NodeState::Active => (),
                NodeState::Terminated => return Ok(()),

                NodeState::Inactive => {
                    if !active {
                        self._result_tx.send(ThreadResult::Inactive {
                            from: self._node.get_id(),
                        })?;

                        while let NodeState::Inactive = state {
                            self._node.wait_for_signal();

                            state = self._node.get_state();
                            match state {
                                NodeState::Active => {
                                    self._result_tx.send(ThreadResult::Active {
                                        from: self._node.get_id(),
                                    })?
                                }
                                NodeState::Terminated => return Ok(()),
                                NodeState::Inactive => {}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Switch {
    _nodes: Vec<Node>,
    _result_rx: Receiver<ThreadResult>,
}

impl Switch {
    pub fn new(code: Code, count: usize) -> Switch {
        let mut nodes = Vec::with_capacity(count);

        let (result_tx, result_rx) = channel();

        for number in 0..count {
            let thread_result_tx = result_tx.clone();
            let code = code.clone();
            let node = Node::new(number);
            nodes.push(node.clone());

            thread::spawn(move || {
                let mut vm = NodeVm::new(code.clone(), node, thread_result_tx);
                if let Err(err) = vm.run() {
                    println!("{:2} Error: {:?}", number, err);
                }
            });
        }

        Switch {
            _nodes: nodes,
            _result_rx: result_rx,
        }
    }

    pub fn part1(&mut self) -> Result<i64, NetworkError> {
        loop {
            let thread_result = self._result_rx.recv()?;
            match thread_result {
                ThreadResult::Result { to, x, y, .. } => match to {
                    0..=49 => {
                        self._nodes[to].feed(x, y);
                    }
                    255 => {
                        return Ok(y);
                    }
                    _ => return Err(NetworkError::UnknownAddress(to)),
                },

                ThreadResult::Inactive { .. } => {}
                ThreadResult::Active { .. } => {}
            }
        }
    }

    pub fn part2(&mut self) -> Result<i64, NetworkError> {
        use ThreadResult::*;

        let mut nat_memory = None;
        let mut last_delivered = None;
        let mut inactive = HashSet::new();

        loop {
            loop {
                let thread_result = self._result_rx.try_recv();
                match thread_result {
                    Ok(Result { to, x, y, .. }) => match to {
                        0..=49 => {
                            self._nodes[to].feed(x, y);
                        }
                        255 => nat_memory = Some((x, y)),

                        _ => return Err(NetworkError::UnknownAddress(to)),
                    },

                    Ok(Inactive { from }) => {
                        inactive.insert(from);
                    }

                    Ok(Active { from }) => {
                        inactive.remove(&from);
                    }

                    Err(TryRecvError::Empty) => {
                        if inactive.len() == 50 {
                            break;
                        }
                    }
                    Err(TryRecvError::Disconnected) => return Err(NetworkError::NodeStopped),
                }
            }

            if let Some((x, y)) = nat_memory {
                if last_delivered.map_or(false, |old_y| old_y == y) {
                    return Ok(y);
                }
                inactive.remove(&0);
                self._nodes[0].feed(x, y);
                last_delivered = Some(y);
            }
        }
    }
}

impl Drop for Switch {
    fn drop(&mut self) {
        for node in &self._nodes {
            node.terminate();
        }
    }
}
