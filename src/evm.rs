use crate::Calldata;
use crate::Memory;
use crate::Stack;
use crate::Storage;

#[derive(Debug)]
pub struct EVM {
    bytecode: String,
    opcode_index: u64,

    stack: Stack,
    memory: Memory,
    calldata: Calldata,
    storage: Storage,
}

impl EVM {
    pub fn default(bytecode: &str) -> EVM {
        EVM {
            bytecode: String::from(bytecode),
            opcode_index: 0,
            stack: Stack::default(),
            memory: Memory::default(),
            calldata: Calldata::default(),
            storage: Storage::default(),
        }
    }

    pub fn new(
        bytecode: &str,
        calldata: Calldata,
        memory: Option<Memory>,
        storage: Option<Storage>,
        stack: Option<Stack>,
    ) -> EVM {
        let defined_memory: Memory;
        let defined_storage: Storage;
        let defined_stack: Stack;

        match memory {
            Some(value) => {
                defined_memory = value;
            }

            None => {
                defined_memory = Memory::default();
            }
        }

        match storage {
            Some(value) => {
                defined_storage = value;
            }

            None => {
                defined_storage = Storage::default();
            }
        }

        match stack {
            Some(value) => {
                defined_stack = value;
            }

            None => {
                defined_stack = Stack::default();
            }
        }

        EVM {
            bytecode: String::from(bytecode),
            calldata,
            opcode_index: 0,
            memory: defined_memory,
            storage: defined_storage,
            stack: defined_stack,
        }
    }
}
