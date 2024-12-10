use crate::utils::operations::pad_32_bytes;

#[derive(Debug)]
pub struct Stack {
    pub trace: Vec<String>,
}

impl Stack {
    pub fn default() -> Stack {
        Stack { trace: Vec::new() }
    }

    pub fn push(&mut self, value: &str) -> Result<(), String> {
        self.trace.push(pad_32_bytes(&value));
        Ok(())
    }

    // fn pop(&self) -> Result<()> {
    //     // remove element from stack
    // }
}
