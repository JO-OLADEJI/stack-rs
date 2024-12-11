use crate::utils::operations::validate_calldata;

#[derive(Debug)]
pub struct Calldata {
    buffer: String,
}

impl Calldata {
    pub fn default() -> Calldata {
        Calldata {
            buffer: String::from("0x"),
        }
    }

    pub fn new(value: &str) -> Result<Calldata, String> {
        match validate_calldata(&value) {
            Some(fmt_data) => Ok(Calldata { buffer: fmt_data }),
            _ => Err(String::from("Invalid calldata")),
        }
    }
}
