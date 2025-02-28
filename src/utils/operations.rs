use crate::{evm::EVM, utils::constants::Constants};

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

pub fn convert_hex_to_dec(hex_str: &str) -> usize {
    if let Ok(value) = usize::from_str_radix(hex_str, 16) {
        return value;
    } else {
        // TODO: this should throw an error
        return 0;
    }
}
