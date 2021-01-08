pub struct Modes {
    modes: Vec<u8>,
}

impl Modes {
    pub fn new(instruction: i32) -> Modes {
        let mut modes = Vec::new();
        let mut instruction = instruction;
        while instruction > 0 {
            modes.push((instruction % 10) as u8);
            instruction /= 10;
        }
        Modes { modes }
    }

    pub fn get(&self, pos: usize) -> u8 {
        *self.modes.get(pos).unwrap_or(&0)
    }
}
