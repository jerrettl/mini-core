use super::{control_signals::*, memory::Memory, registers::Registers};
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

    current_stage: Stage,
}

#[derive(Default, Copy, Clone, Eq, PartialEq)]
enum Stage {
    #[default]
    InstructionFetch,
    InstructionDecode,
    Execute,
    Memory,
    WriteBack,
}

impl Stage {
    fn get_next_stage(current_stage: Stage) -> Stage {
        match current_stage {
            Stage::InstructionFetch => Stage::InstructionDecode,
            Stage::InstructionDecode => Stage::Execute,
            Stage::Execute => Stage::Memory,
            Stage::Memory => Stage::WriteBack,
            Stage::WriteBack => Stage::InstructionFetch,
        }
    }
}

fn error(message: &str) {
    panic!("{}", message);
}

impl Datapath for MipsDatapath {
    fn execute_instruction(&mut self) {
        println!("Running an instruction!");

        // If the last instruction has not finished, finish it instead.
        if self.current_stage != Stage::InstructionFetch {
            self.finish_instruction();
            return;
        }

        // IF
        self.stage_instruction_fetch();

        // ID
        self.stage_instruction_decode();

        // EX
        self.stage_execute();

        // MEM
        self.stage_memory();

        // WB
        self.stage_writeback();
    }

    fn execute_stage(&mut self) {
        match self.current_stage {
            Stage::InstructionFetch => self.stage_instruction_fetch(),
            Stage::InstructionDecode => self.stage_instruction_decode(),
            Stage::Execute => self.stage_execute(),
            Stage::Memory => self.stage_memory(),
            Stage::WriteBack => self.stage_writeback(),
        }

        self.current_stage = Stage::get_next_stage(self.current_stage);
    }

    fn get_register(&self, register: &str) -> Option<u64> {
        Some(self.registers[register])
    }
}

impl MipsDatapath {
    fn finish_instruction(&mut self) {
        while self.current_stage != Stage::InstructionFetch {
            self.execute_stage();
        }
    }

    fn stage_instruction_fetch(&mut self) {
        self.instruction_fetch();
    }

    fn stage_instruction_decode(&mut self) {
        self.instruction_decode();
        self.sign_extend();
        self.set_control_signals();
        self.read_registers();
        self.set_alu_control();
    }

    fn stage_execute(&mut self) {
        self.alu();
    }

    fn stage_memory(&mut self) {
        /*
        if self.signals.mem_read == 1{
            self.memory_read();
        }

        if self.signals.mem_write == 1 {
            self.memory_write();
        }
        */
    }

    fn stage_writeback(&mut self) {
        self.register_write();
        self.set_pc();
    }

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
                self.signals.alu_op = AluOp::UseFunctField;
                self.signals.alu_src = AluSrc::ReadRegister2;
                self.signals.branch = Branch::NoBranch;
                self.signals.jump = Jump::NoJump;
                self.signals.mem_read = MemRead::NoRead;
                self.signals.mem_to_reg = MemToReg::UseAlu;
                self.signals.mem_write = MemWrite::NoWrite;
                self.signals.mem_write_src = MemWriteSrc::PrimaryUnit;
                self.signals.reg_dst = RegDst::Reg3;
                self.signals.reg_write = RegWrite::YesWrite;
            }
            _ => error("Instruction not supported."),
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
            AluOp::Addition => AluControl::Addition,
            AluOp::Subtraction => AluControl::Subtraction,
            AluOp::SetOnLessThanSigned => AluControl::SetOnLessThanSigned,
            AluOp::SetOnLessThanUnsigned => AluControl::SetOnLessThanUnsigned,
            AluOp::And => AluControl::And,
            AluOp::Or => AluControl::Or,
            AluOp::LeftShift16 => AluControl::LeftShift16,
            AluOp::UseFunctField => match self.funct {
                0b100000 => AluControl::Addition,
                0b100010 => AluControl::Subtraction,
                0b100100 => AluControl::And,
                0b100101 => AluControl::Or,
                0b101010 => AluControl::SetOnLessThanSigned,
                0b101011 => AluControl::SetOnLessThanUnsigned,
                _ => {
                    error("Unsupported funct");
                    AluControl::Addition // Stub
                }
            },
        };
    }

    fn alu(&mut self) {
        let input1 = self.read_data_1;
        let input2 = match self.signals.alu_src {
            AluSrc::ReadRegister2 => self.read_data_2,
            AluSrc::ExtendedImmediate => self.sign_extend,
        };

        self.alu_result = match self.signals.alu_control {
            AluControl::Addition => input1 + input2,
            AluControl::Subtraction => input1 - input2,
            AluControl::SetOnLessThanSigned => ((input1 as i64) < (input2 as i64)) as u64,
            AluControl::SetOnLessThanUnsigned => (input1 < input2) as u64,
            AluControl::And => input1 & input2,
            AluControl::Or => input1 | input2,
            AluControl::LeftShift16 => input2 << 16,
            AluControl::Not => !input1,
        }
    }

    fn register_write(&mut self) {
        self.data_result = match self.signals.mem_to_reg {
            MemToReg::UseAlu => self.alu_result,
            MemToReg::UseMemory => self.memory_data,
        };

        if self.signals.reg_write == RegWrite::NoWrite {
            return;
        }

        let destination = match self.signals.reg_dst {
            RegDst::Reg2 => self.rt as usize,
            RegDst::Reg3 => self.rd as usize,
        };

        self.registers.gpr[destination] = self.data_result;
    }

    fn set_pc(&mut self) {
        self.registers.pc += 4;
    }
}
