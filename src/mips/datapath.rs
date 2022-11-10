use super::{memory::Memory, registers::Registers};
use crate::datapath::Datapath;

#[derive(Default)]
pub struct MipsDatapath {
    pub registers: Registers,
    pub memory: Memory,
    pub instruction: u32,
    pub signals: ControlSignals,

    opcode: u32,
    rs: u32,
    rt: u32,
    rd: u32,
    shamt: u32,
    funct: u32,
    imm: u32,

    read_data_1: u64,
    read_data_2: u64,
    sign_extend: u64,

    alu_result: u64,
    memory_data: u64,
    data_result: u64,
}

#[derive(Default)]
pub struct ControlSignals {
    alu_control: u8,
    alu_op: u8,
    alu_src: u8,
    branch: u8,
    jump: u8,
    mem_read: u8,
    mem_to_reg: u8,
    mem_write: u8,
    mem_write_src: u8,
    reg_dst: u8,
    reg_write: u8,
}

impl Datapath for MipsDatapath {
    fn execute_instruction(&mut self) {
        println!("Running an instruction!");

        // IF
        self.instruction_fetch();

        // ID
        self.instruction_decode();
        self.sign_extend();
        self.set_control_signals();
        self.read_registers();
        self.set_alu_control();

        // EX
        self.alu();

        // MEM
        /*
        if self.signals.mem_read == 1{
            self.memory_read();
        }

        if self.signals.mem_write == 1 {
            self.memory_write();
        }
        */

        // WB
        self.register_write();
        self.set_pc();
    }

    fn get_register(&self, register: &str) -> Option<u64> {
        match register {
            "pc" => Some(self.registers.pc),
            _ => None,
        }
    }
}

impl MipsDatapath {
    fn instruction_fetch(&mut self) {
        // Load instruction
        self.instruction = self.memory.load_word(self.registers.pc);
    }

    fn instruction_decode(&mut self) {
        self.opcode = (self.instruction >> 26) & 0b111111;
        self.rs = (self.instruction >> 21) & 0b11111;
        self.rt = (self.instruction >> 16) & 0b11111;
        self.rd = (self.instruction >> 11) & 0b11111;
        self.shamt = (self.instruction >> 6) & 0b11111;
        self.funct = self.instruction & 0b111111;
        self.imm = self.instruction & 0xFFFF;
    }

    fn sign_extend(&mut self) {
        // Is the value negative or positive? Check sign bit

        // 0000 0000 0000 0000 1000 0000 0000 0000
        // 0x00008000

        self.sign_extend = if (self.imm & 0x00008000) >> 15 == 0 {
            self.imm as u64
        } else {
            (self.imm as u64) | 0xFFFF_FFFF_FFFF_0000
        }
    }

    fn set_control_signals(&mut self) {
        match self.opcode {
            // R-type instructions (add, sub, and, or, slt, sltu)
            0 => {
                self.signals.alu_op = 7;
                self.signals.alu_src = 0;
                self.signals.branch = 0;
                self.signals.jump = 0;
                self.signals.mem_read = 0;
                self.signals.mem_to_reg = 0;
                self.signals.mem_write = 0;
                self.signals.mem_write_src = 0;
                self.signals.reg_dst = 1;
                self.signals.reg_write = 1;
            }
            _ => panic!("Instruction not supported."),
        }
    }

    fn read_registers(&mut self) {
        let reg1 = self.rs as usize;
        let reg2 = self.rt as usize;

        self.read_data_1 = self.registers.gpr[reg1];
        self.read_data_2 = self.registers.gpr[reg2];
    }

    fn set_alu_control(&mut self) {
        self.signals.alu_control = match self.signals.alu_op {
            0..=6 => self.signals.alu_op,
            7 => match self.funct {
                0b100000 => 0, // add
                0b100010 => 1, // sub
                0b100100 => 4, // and
                0b100101 => 5, // or
                0b101010 => 2, // slt
                0b101011 => 3, // sltu
                _ => panic!("Unsupported funct"),
            },
            _ => panic!("Invalid ALUOp control signal"),
        };
    }

    fn alu(&mut self) {
        let input1 = self.read_data_1;
        let input2 = match self.signals.alu_src {
            0 => self.read_data_2,
            1 => self.sign_extend,
            _ => panic!("Invalid ALUSrc control signal"),
        };

        self.alu_result = match self.signals.alu_control {
            0 => input1 + input2,
            1 => input1 - input2,
            2 => {
                if (input1 as i64) < (input2 as i64) {
                    1
                } else {
                    0
                }
            }
            3 => {
                if input1 < input2 {
                    1
                } else {
                    0
                }
            }
            4 => input1 & input2,
            5 => input1 | input2,
            6 => input2 << 16,
            7 => !input1,
            _ => panic!("Bad ALU control signal"),
        }
    }

    fn register_write(&mut self) {
        self.data_result = match self.signals.mem_to_reg {
            0 => self.alu_result,
            1 => self.memory_data,
            _ => panic!("Invalid MemToReg control signal"),
        };

        if self.signals.reg_write == 0 {
            return;
        }

        let destination = match self.signals.reg_dst {
            0 => self.rt as usize,
            1 => self.rd as usize,
            _ => panic!("Invalid RegDst control signal"),
        };

        self.registers.gpr[destination] = self.data_result;
    }

    fn set_pc(&mut self) {
        self.registers.pc += 4;
    }
}
