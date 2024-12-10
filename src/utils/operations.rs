pub fn pad_32_bytes(value: &str) -> String {
    let hex_prefix = "0x";
    let bytes_32_length = 64;
    let mut raw_value = String::new();
    let mut padded_value = String::from(hex_prefix);

    if value.starts_with(hex_prefix) {
        raw_value.push_str(&value[hex_prefix.len()..]);
    } else {
        raw_value.push_str(value);
    }

    if raw_value.len() > bytes_32_length {
        padded_value.push_str(&raw_value[..bytes_32_length]);
    } else {
        padded_value.push_str("0".repeat(bytes_32_length - raw_value.len()).as_str());
    }

    padded_value.push_str(&raw_value);
    padded_value
}
