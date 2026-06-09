mod cpu;
mod bus;

use cpu::Cpu;
use bus::Bus;

fn main() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();


    bus.write_byte(0x1000, 0xFF);
    bus.write_byte(0x1001, 0xAA);
    bus.write_byte(0x1002, 0x55);
    bus.dump_memory(0x1000, 3);


    cpu.reset();
    println!("CPU was Reset");
}