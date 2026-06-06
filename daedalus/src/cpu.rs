pub struct Cpu {
    pub a: u8,
    pub pc: u16,
}


impl Cpu {

    pub fn new() -> Self {
        Self {
            a: 0,
            pc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.pc = 0;
    }
}