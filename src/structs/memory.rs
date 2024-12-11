#[derive(Debug)]
pub struct Memory {
    data: String,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: String::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        // TODO: properly calculate the size with stripped `0x`
        // NOTE: 2 characters make one byte
        self.data.len()
    }
}
