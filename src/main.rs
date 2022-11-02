pub mod mips;

use mips::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();

    cpu.run();
}
