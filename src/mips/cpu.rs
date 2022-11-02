pub struct Cpu {
    pub pc: u64,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        println!("Hello I am a cpu!");
        println!("PC: {}", self.pc);
    }
}
