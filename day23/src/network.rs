use crate::error::NetworkError;
use computer::{Code, ComputerInput, VirtualMachine};
use mpsc::{channel, TryRecvError};
use std::{
    collections::{HashSet, VecDeque},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Condvar, Mutex,
    },
    thread,
};

const EMPTY_THRESHOLD: usize = 2;

#[derive(Debug)]
enum NodeState {
    Active,
    Inactive,
    Terminated,
}

#[derive(Debug)]
struct NodeData {
    queue: VecDeque<i64>,
    empty_for: usize,
    terminated: bool,
}

impl NodeData {
    fn assess(&self) -> NodeState {
        if self.terminated {
            NodeState::Terminated
        } else if !self.queue.is_empty() || self.empty_for < EMPTY_THRESHOLD {
            NodeState::Active
        } else {
            NodeState::Inactive
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    data: Arc<(Mutex<NodeData>, Condvar)>,
}

impl Node {
    pub fn new(id: usize) -> Node {
        let mut queue = VecDeque::new();
        queue.push_back(id as i64);
        let data = NodeData {
            queue,
            empty_for: 0,
            terminated: false,
        };
        Node {
            id,
            data: Arc::new((Mutex::new(data), Condvar::new())),
        }
    }

    pub fn get_id(&self) -> usize {
        return self.id;
    }

    pub fn feed(&self, x: i64, y: i64) {
        let (guard, wake_upper) = &*self.data;
        let mut data = guard.lock().unwrap();
        data.queue.push_back(x);
        data.queue.push_back(y);
        wake_upper.notify_one();
    }

    pub fn terminate(&self) {
        let (guard, wake_upper) = &*self.data;
        let mut data = guard.lock().unwrap();
        data.terminated = true;
        wake_upper.notify_one();
    }

    pub fn get_state(&self) -> NodeState {
        let (guard, _) = &*self.data;
        let data = guard.lock().unwrap();
        data.assess()
    }

    pub fn wait_for_signal(&self) {
        let (guard, wake_upper) = &*self.data;
        let mut data = guard.lock().unwrap();
        while let NodeState::Inactive = data.assess() {
            data = wake_upper.wait(data).unwrap();
        }
    }
}

impl ComputerInput for Node {
    fn get_next_input(&mut self) -> Option<i64> {
        let (guard, _) = &*self.data;
        let mut data = guard.lock().unwrap();
        if let Some(value) = data.queue.pop_front() {
            data.empty_for = 0;
            Some(value)
        } else {
            data.empty_for += 1;
            Some(-1)
        }
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
}

#[derive(Debug)]
struct NodeVm<'a> {
    id: usize,
    vm: VirtualMachine<'a>,
    node: Node,

    result_tx: Sender<ThreadResult>,

    next_receiver: Option<usize>,
    next_x: Option<i64>,
}

impl<'a> NodeVm<'a> {
    pub fn new(code: Code, node: Node, result_tx: Sender<ThreadResult>) -> NodeVm<'a> {
        let vm = VirtualMachine::new_with_id(code, node.clone(), node.get_id());

        NodeVm {
            id: node.get_id(),
            vm,
            node,

            result_tx,

            next_receiver: None,
            next_x: None,
        }
    }

    pub fn run(&mut self) -> Result<(), NetworkError> {
        loop {
            let active = match self.vm.step()? {
                computer::StepResult::Value(value) => {
                    if self.next_receiver.is_none() {
                        self.next_receiver = Some(value as usize);
                    } else if self.next_x.is_none() {
                        self.next_x = Some(value);
                    } else {
                        let result = ThreadResult::Result {
                            from: self.node.id,
                            to: self.next_receiver.unwrap(),
                            x: self.next_x.unwrap(),
                            y: value,
                        };
                        self.next_receiver = None;
                        self.next_x = None;
                        self.result_tx.send(result)?;
                    };
                    true
                }
                computer::StepResult::Stop => return Err(NetworkError::NodeStopped),
                computer::StepResult::Proceed => false,
                computer::StepResult::WaitForInput => false,
            };

            match self.node.get_state() {
                NodeState::Active => (),
                NodeState::Terminated => return Ok(()),

                NodeState::Inactive => {
                    if !active {
                        self.result_tx.send(ThreadResult::Inactive {
                            from: self.node.get_id(),
                        })?;

                        loop {
                            self.node.wait_for_signal();

                            match self.node.get_state() {
                                NodeState::Active => break,
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
    nodes: Vec<Node>,
    result_rx: Receiver<ThreadResult>,
}

impl Switch {
    pub fn part1(code: Code, num_nodes: usize) -> Result<i64, NetworkError> {
        let switch = Switch::start_nodes(code, num_nodes);

        loop {
            if let ThreadResult::Result { to, x, y, .. } = switch.result_rx.recv()? {
                match to {
                    to if to < num_nodes => switch.nodes[to].feed(x, y),
                    255 => return Ok(y),
                    _ => return Err(NetworkError::UnknownAddress(to)),
                }
            }
        }
    }

    pub fn part2(code: Code, num_nodes: usize) -> Result<i64, NetworkError> {
        use ThreadResult::*;

        let switch = Switch::start_nodes(code, num_nodes);

        let mut nat_memory = None;
        let mut last_delivered = None;
        let mut active = (0..num_nodes).collect::<HashSet<usize>>();

        loop {
            match switch.result_rx.try_recv() {
                Ok(Result { to, x, y, .. }) => match to {
                    to if to < num_nodes => {
                        active.insert(to);
                        switch.nodes[to].feed(x, y)
                    }
                    255 => nat_memory = Some((x, y)),
                    _ => return Err(NetworkError::UnknownAddress(to)),
                },

                Ok(Inactive { from }) => {
                    active.remove(&from);
                }

                Err(TryRecvError::Empty) => {
                    if active.is_empty() {
                        if let Some((x, y)) = nat_memory {
                            if last_delivered.map_or(false, |old_y| old_y == y) {
                                return Ok(y);
                            }
                            active.insert(0);
                            switch.nodes[0].feed(x, y);
                            last_delivered = Some(y);
                        }
                    }
                }

                Err(TryRecvError::Disconnected) => return Err(NetworkError::NodeStopped),
            }
        }
    }

    fn start_nodes(code: Code, num_nodes: usize) -> Switch {
        let mut nodes = Vec::with_capacity(num_nodes);

        let (result_tx, result_rx) = channel();

        for id in 0..num_nodes {
            let code = code.clone();
            let node = Node::new(id);
            nodes.push(node.clone());
            let thread_result_tx = result_tx.clone();

            thread::spawn(move || {
                let mut vm = NodeVm::new(code, node, thread_result_tx);
                if let Err(err) = vm.run() {
                    println!("{:2} Error: {:?}", id, err);
                }
            });
        }

        Switch { nodes, result_rx }
    }
}

impl Drop for Switch {
    fn drop(&mut self) {
        for node in &self.nodes {
            node.terminate();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), NetworkError> {
        let code = Code::from_file("day23", "input.txt")?;
        let result = Switch::part1(code, 50)?;
        let expected = 22659;
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), NetworkError> {
        let code = Code::from_file("day23", "input.txt")?;
        let result = Switch::part2(code, 50)?;
        let expected = 17429;
        assert_eq!(expected, result);

        Ok(())
    }
}
