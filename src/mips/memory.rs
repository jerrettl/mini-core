// const BYTES_4K: usize = 4 * 1024;
const CAPACITY: usize = 16;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: vec![0; CAPACITY],
        }
    }
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, value) in self.memory.iter().enumerate() {
            if i % 4 == 0 {
                write!(f, "{:04}   ", i)?;
            }
            write!(f, "{:08b}", value)?;
            if i % 4 < 3 {
                write!(f, " ")?;
            } else if i % 4 == 3 && i > 0 && i < self.memory.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Memory {
    // Assume a proper address for now.
    // A word is 32 bits.
    pub fn store_word(&mut self, address: u64, data: u32) {
        println!("Storing {data} at {address}");
        let address = address as usize;
        self.memory[address] = ((data >> 24) & 0b11111111) as u8;
        self.memory[address + 1] = ((data >> 16) & 0b11111111) as u8;
        self.memory[address + 2] = ((data >> 8) & 0b11111111) as u8;
        self.memory[address + 3] = (data & 0b11111111) as u8;
    }

    // Assume a proper address for now.
    // A word is 32 bits.
    pub fn load_word(&self, address: u64) -> u32 {
        let address = address as usize;
        let mut result: u32 = 0;

        result |= (self.memory[address] as u32) << 24;
        result |= (self.memory[address + 1] as u32) << 16;
        result |= (self.memory[address + 2] as u32) << 8;
        result |= self.memory[address + 3] as u32;

        result
    }
}
