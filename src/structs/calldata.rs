use crate::utils::{
    constants::Constants,
    operations::{pad_32_bytes, validate_calldata},
};

#[derive(Debug)]
pub struct Calldata {
    pub buffer: String,
}

impl Calldata {
    pub fn default() -> Calldata {
        Calldata {
            buffer: String::from(""),
        }
    }

    pub fn new(value: &str) -> Calldata {
        match validate_calldata(&value) {
            Some(fmt_data) => Calldata { buffer: fmt_data },
            // TODO: add an option for a flag to terminate process if calldata is invalid
            None => Calldata {
                buffer: String::from(""),
            },
        }
    }

    pub fn read(&self, bytes_index: usize) -> String {
        let raw_buffer = &self.buffer;

        if raw_buffer.len() < (bytes_index * Constants::BYTE_SIZE()) {
            pad_32_bytes("")
        } else {
            pad_32_bytes(&raw_buffer[bytes_index * Constants::BYTE_SIZE()..])
        }
    }
}
