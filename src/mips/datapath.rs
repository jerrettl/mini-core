use super::{memory::Memory, registers::Registers};
use crate::datapath::Datapath;

#[derive(Default)]
pub struct MipsDatapath {
    pub registers: Registers,
    pub memory: Memory,
}

impl Datapath for MipsDatapath {
    fn execute_instruction(&mut self) {
        println!("Running an instruction!");
        // Load instruction
        let instruction: u32 = self.memory.load_word(self.registers.pc);

        // Partition instruction
        let op: u32 = (instruction >> 26) & 0b111111;
        let rs: u32 = (instruction >> 21) & 0b11111;
        let rt: u32 = (instruction >> 16) & 0b11111;
        let rd: u32 = (instruction >> 11) & 0b11111;
        // let shamt: u32 = (instruction >> 6) & 0b11111;
        let funct: u32 = instruction & 0b111111;

        // Read registers
        let read_data_1: u64 = self.registers.gpr[rs as usize];
        let read_data_2: u64 = self.registers.gpr[rt as usize];

        // Perform R-type instruction.
        let result: u64 = if op == 0 {
            match funct {
                0b100000 => read_data_1 + read_data_2,
                _ => 0,
            }
        } else {
            0
        };

        // Write to register.
        self.registers.gpr[rd as usize] = result as u64;

        // PC + 4
        self.registers.pc += 4;
    }

    fn get_register(&self, register: &str) -> Option<u64> {
        match register {
            "pc" => Some(self.registers.pc),
            _ => None,
        }
    }
}
