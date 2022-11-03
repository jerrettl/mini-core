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
        self.registers.pc += 4;
    }

    fn get_register(&self, register: &str) -> Option<u64> {
        match register {
            "pc" => Some(self.registers.pc),
            _ => None,
        }
    }
}
