mod cpu;

use cpu::Cpu;


fn main() {
    let mut cpu = Cpu::new();

    println!("A  = {}", cpu.a);
    println!("PC = {}", cpu.pc);

    cpu.reset();

    println!("CPU was Reset");
}