pub mod datapath;
pub mod mips;

use datapath::Datapath;
use mips::datapath::MipsDatapath;

fn main() {
    let mut datapath = MipsDatapath::default();

    // println!("{:#?}", datapath.registers);
    println!("PC: {}", datapath.get_register("pc").unwrap());
    datapath.execute_instruction();
    // println!("{:#?}", datapath.registers);
    println!("PC: {}", datapath.get_register("pc").unwrap());
}
