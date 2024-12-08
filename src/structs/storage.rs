use std::collections::HashMap;

#[derive(Debug)]
pub struct Storage {
    slots: HashMap<String, String>,
}

impl Storage {
    pub fn default() -> Storage {
        Storage {
            slots: HashMap::new(),
        }
    }
}
