use crate::Calldata;
use crate::Memory;
use crate::Stack;
use crate::Storage;

pub enum BytecodeExecutionTrail {
    Left(usize),
    Right(usize),
}

#[derive(Debug)]
pub struct EVM {
    bytecode: String,
    opcode_index: usize,

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

    pub fn get_bytecode(&self) -> &String {
        &self.bytecode
    }

    pub fn get_opcode_index(&self) -> usize {
        self.opcode_index
    }

    pub fn update_opcode_index(&mut self, command: BytecodeExecutionTrail) {
        match command {
            BytecodeExecutionTrail::Left(magnitude) => {
                if magnitude > self.opcode_index {
                    self.opcode_index = 0
                } else {
                    self.opcode_index -= magnitude
                };
            }

            BytecodeExecutionTrail::Right(magnitude) => {
                if self.opcode_index + magnitude > self.bytecode.len() {
                    self.opcode_index = self.bytecode.len()
                } else {
                    self.opcode_index += magnitude
                }
            }
        }
    }
}
