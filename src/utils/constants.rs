use regex::Regex;

pub struct Constants;

impl Constants {
    pub fn HEX_PREFIX() -> String {
        return String::from("0x");
    }

    pub fn BYTES_32_SIZE() -> usize {
        return 64;
    }

    pub fn BYTE_SIZE() -> usize {
        return 2;
    }

    pub fn OPCODE_LENGTH() -> usize {
        return 2;
    }

    pub fn HEX_REGEX() -> Regex {
        return Regex::new(r"^[0-9a-fA-F]+$").unwrap();
    }
}
