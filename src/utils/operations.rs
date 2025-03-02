use crate::{evm::EVM, utils::constants::Constants};
use num_bigint::BigUint;

pub fn pad_32_bytes(value: &str) -> String {
    if value.len() > Constants::BYTES_32_SIZE() {
        value[..Constants::BYTES_32_SIZE()].to_string()
    } else {
        format!(
            "{}{}",
            "0".repeat(Constants::BYTES_32_SIZE() - &value.len())
                .as_str(),
            value
        )
    }
}

pub fn validate_calldata(value: &str) -> Option<String> {
    // TODO: handle function dispatcher bug when calldata is 4 bytes (func selector)
    let mut raw_calldata = String::from("");

    if value.is_empty() {
        return Some(raw_calldata);
    } else {
        raw_calldata = if value.starts_with(&Constants::HEX_PREFIX()) {
            value[Constants::HEX_PREFIX().len()..].to_string()
        } else {
            value.to_string()
        };

        if Constants::HEX_REGEX().is_match(&raw_calldata) {
            return Some(raw_calldata);
        }

        return None;
    }
}

pub fn get_payload(execution_context: &EVM, bytes_count: usize) -> String {
    let pc = execution_context.get_program_counter() + execution_context.get_command_length();
    execution_context.get_bytecode()[pc..pc + (bytes_count * Constants::OPCODE_LENGTH())]
        .to_string()
}

pub fn convert_hex_to_dec(hex: &str) -> usize {
    if let Ok(value) = usize::from_str_radix(hex, 16) {
        return value;
    } else {
        // TODO: this should throw an error
        return 0;
    }
}

pub fn convert_hex_to_biguint(hex: &str) -> BigUint {
    if Constants::HEX_REGEX().is_match(&hex) {
        let char_vec: Vec<char> = hex.chars().collect();
        let u8_vec: Vec<u8> = char_vec
            .iter()
            .filter_map(|byte| u8::from_str_radix(&byte.to_string(), 16).ok())
            .collect();

        return BigUint::from_radix_be(&u8_vec, 16).unwrap();
    }
    // TODO: this should throw an error
    return BigUint::ZERO;
}

pub fn convert_biguint_to_hex(value: &BigUint) -> String {
    return value.to_str_radix(16);
}
