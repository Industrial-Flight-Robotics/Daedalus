mod cpu;

use cpu::Cpu;


fn main() {
    let mut cpu = Cpu::new();

    println!("A  = {:#04X}", cpu.a);
    println!("B  = {:#04X}", cpu.b);
    println!("PC = {:#06X}", cpu.pc);
    println!("SP = {:#06X}", cpu.sp);

    cpu.reset();

    println!("CPU was Reset");
}