use common::as_int;

#[derive(Clone)]
pub struct Computer {
    _memory: Vec<i32>,
    _pointer: usize,
}

impl<'a> Computer {
    pub fn new(code: Vec<i32>) -> Computer {
        Computer {
            _memory: code.clone(),
            _pointer: 0,
        }
    }

    pub fn parse(input: &str) -> Computer {
        let parts = input.split(",").map(as_int).collect();
        Computer::new(parts)
    }

    pub fn patch_memory(&mut self, pos: usize, value: i32) {
        self._memory[pos] = value;
    }

    pub fn memory(&self) -> &Vec<i32> {
        &self._memory
    }

    pub fn run(&mut self) -> &Computer {
        let mut opcode = self._memory[self._pointer];

        while opcode != 99 {
            match opcode {
                1 => self.add(),
                2 => self.mul(),

                _ => panic!("Unknown op"),
            }
            opcode = self._memory[self._pointer];
        }

        self
    }

    fn get_value(&self, pos: usize) -> i32 {
        let addr = self._memory[pos as usize] as usize;
        self._memory[addr]
    }

    fn get_addr(&self, pos: usize) -> usize {
        return self._memory[pos] as usize;
    }

    fn add(&mut self) {
        let op1 = self.get_value(self._pointer + 1);
        let op2 = self.get_value(self._pointer + 2);
        let result = self.get_addr(self._pointer + 3);

        self._memory[result] = op1 + op2;

        self._pointer += 4;
    }

    fn mul(&mut self) {
        let op1 = self.get_value(self._pointer + 1);
        let op2 = self.get_value(self._pointer + 2);
        let result = self.get_addr(self._pointer + 3);

        self._memory[result] = op1 * op2;

        self._pointer += 4;
    }
}
