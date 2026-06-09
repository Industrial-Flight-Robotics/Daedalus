pub struct Cpu {
    pub a: u8,
    pub b: u8,
    pub pc: u16,
    pub sp: u16,
}


impl Cpu {

    pub fn new() -> Self {
        Self {
            a: 0x00,
            b: 0x00,
            pc: 0x0000,
            sp: 0x0000,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0x00;
        self.b = 0x00;
        self.pc = 0x0000;
        self.sp = 0x0000;
    }

    pub fn state(&self) {
        println!("A  = {:#04X}", self.a);
        println!("B  = {:#04X}", self.b);
        println!("PC = {:#06X}", self.pc);
        println!("SP = {:#06X}", self.sp);
    }
    
}