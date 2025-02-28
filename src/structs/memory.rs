#[derive(Debug)]
pub struct Memory {
    data: String,
}

impl Memory {
    pub fn default() -> Memory {
        Memory {
            data: String::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }
}
