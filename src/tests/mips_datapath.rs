use crate::datapath::Datapath;
use crate::mips::datapath::MipsDatapath;
use crate::mips::registers::RegisterType;

#[allow(clippy::unusual_byte_groupings)]
#[test]
fn double_register() {
    let mut datapath = MipsDatapath::default();

    let instruction: u32 = 0b000000_01001_01001_01001_00000_100000;
    datapath.memory.store_word(0, instruction);
    datapath.registers[RegisterType::T1] = 5;

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[9], 10);
}
