const BYTES_4K: usize = 4 * 1024;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        const CAPACITY: usize = BYTES_4K;

        let memory: Vec<u8> = vec![0; CAPACITY];

        Self { memory }
    }
}
