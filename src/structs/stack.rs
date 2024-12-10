use crate::utils::operations::pad_32_bytes;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Stack {
    pub trace: Vec<String>,
    undo_trace: VecDeque<Vec<String>>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            trace: Vec::new(),
            undo_trace: VecDeque::new(),
        }
    }

    pub fn undo(&mut self) -> Result<(), String> {
        if let Some(prev_trace) = self.undo_trace.pop_back() {
            self.trace = prev_trace;
        }
        Ok(())
    }

    pub fn push(&mut self, value: &str) -> Result<(), String> {
        self.undo_trace.push_back(self.trace.clone());
        self.trace.push(pad_32_bytes(&value));
        Ok(())
    }

    pub fn pop(&mut self) -> Result<(), String> {
        self.undo_trace.push_back(self.trace.clone());
        // remove element from stack

        Ok(())
    }
}
