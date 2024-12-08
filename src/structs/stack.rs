#[derive(Debug)]
pub struct Stack {
    trace: Vec<String>,
}

impl Stack {
    pub fn default() -> Stack {
        Stack { trace: Vec::new() }
    }

    // fn push(&self) -> Result<()> {
    //     // should expect
    // }
}
