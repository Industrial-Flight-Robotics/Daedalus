pub struct Bus {
    memory: [u8; 65536],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: [0; 65536],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    
    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;
        (high << 8) | low   
    }

     pub fn dump_memory(&self, start: u16, length: usize) {
        for i in 0..length {
            let addr = start as usize + i;

            print!("{:02X} ", self.memory[addr]);
        }
        println!();
    }
}