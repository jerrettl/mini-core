#[derive(Debug, Default)]
pub struct Registers {
    pub pc: u64,
    pub gpr: [u64; 32],
}
