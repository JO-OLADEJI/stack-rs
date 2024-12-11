use crate::utils::constants::Constants;

pub fn pad_32_bytes(value: &str) -> String {
    let mut raw_value = String::new();
    let mut padded_value = String::from(Constants::hex_prefix());

    if value.starts_with(&Constants::hex_prefix()) {
        raw_value.push_str(&value[Constants::hex_prefix().len()..]);
    } else {
        raw_value.push_str(value);
    }

    if raw_value.len() > Constants::bytes_32_size() {
        padded_value.push_str(&raw_value[..Constants::bytes_32_size()]);
    } else {
        padded_value.push_str(
            "0".repeat(Constants::bytes_32_size() - raw_value.len())
                .as_str(),
        );
    }

    padded_value.push_str(&raw_value);
    padded_value
}

pub fn validate_calldata(value: &str) -> Option<String> {
    match value {
        "" => Some(String::from(Constants::hex_prefix())),
        _ => {
            let is_valid: bool;

            if value.starts_with(&Constants::hex_prefix()) {
                is_valid = Constants::hex_regex().is_match(&value[Constants::hex_prefix().len()..]);

                if is_valid {
                    Some(value.to_string())
                } else {
                    None
                }
            } else {
                is_valid = Constants::hex_regex().is_match(&value);

                if is_valid {
                    Some(String::from(Constants::hex_prefix()) + &value.to_string())
                } else {
                    None
                }
            }
        }
    }
}
