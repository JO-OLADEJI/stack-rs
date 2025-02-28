use crate::utils::operations::pad_32_bytes;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Stack {
    pub trace: Vec<String>,
    undo_trace: VecDeque<Vec<String>>,
}

impl Stack {
    pub fn default() -> Stack {
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
        self.trace.pop();
        Ok(())
    }

    // pub fn get_nth_element_from_top(&self, index: usize) -> Option<String> {
    //     // TODO: modify this to use the index parameter
    //     if let Some(top_value) = self.trace.last() {
    //         return Some(top_value.to_string());
    //     } else {
    //         return None;
    //     }
    // }

    pub fn get_top_element(&self) -> Option<String> {
        if let Some(top_value) = self.trace.last() {
            return Some(top_value.to_string());
        } else {
            return None;
        }
    }

    pub fn replace(&mut self, del_count: usize, values: Vec<String>) -> Result<(), String> {
        self.undo_trace.push_back(self.trace.clone());

        for _ in 0..del_count {
            self.trace.pop();
        }

        for i in 0..values.len() {
            self.trace.push(pad_32_bytes(&values[i]));
        }

        Ok(())
    }
}
