#[derive(Debug)]
pub struct Memory {
    data: String,
}

impl Memory {
    pub fn default() -> Memory {
        Memory {
            data: String::from("0x"),
        }
    }
}
