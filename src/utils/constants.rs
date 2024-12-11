use regex::Regex;

pub struct Constants;

impl Constants {
    pub fn hex_prefix() -> String {
        String::from("0x")
    }

    pub fn bytes_32_size() -> usize {
        64
    }

    pub fn hex_regex() -> Regex {
        Regex::new(r"^[0-9a-fA-F]+$").unwrap()
    }
}
