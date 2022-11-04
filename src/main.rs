pub mod datapath;
pub mod mips;

use datapath::Datapath;
use mips::datapath::MipsDatapath;

#[allow(clippy::unusual_byte_groupings)]
fn main() {
    let mut datapath = MipsDatapath::default();

    let instruction: u32 = 0b000000_01001_01001_01001_00000_100000;
    datapath.memory.store_word(0, instruction);
    datapath.registers.gpr[9] = 5;

    // println!("{:b}", 2053);

    println!("PC: {}", datapath.get_register("pc").unwrap());
    println!("Registers:");
    println!("{:#?}", datapath.registers);
    // datapath.memory.memory[1] = 5;
    println!("Memory:");
    println!("{:?}", datapath.memory);

    println!("=======================");
    datapath.execute_instruction();
    println!("=======================");

    // println!("{:#?}", datapath.registers);
    println!("PC: {}", datapath.get_register("pc").unwrap());
    println!("Registers:");
    println!("{:#?}", datapath.registers);
    println!("Memory:");
    println!("{:?}", datapath.memory);
}
