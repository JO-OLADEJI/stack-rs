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

    // fn init() -> Calldata {}
}
