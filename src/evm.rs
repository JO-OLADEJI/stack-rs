use crate::Calldata;
use crate::Memory;
use crate::Stack;
use crate::Storage;
use crate::OPCODES;

pub enum BytecodeExecutionTrail {
    Left,
    Right,
}

#[derive(Debug)]
pub struct EVM {
    bytecode: String,
    program_counter: usize,
    command_length: usize,

    pub stack: Stack,
    pub memory: Memory,
    pub calldata: Calldata,
    pub storage: Storage,
}

impl EVM {
    pub fn default(bytecode: String) -> EVM {
        EVM {
            bytecode,
            command_length: 2,
            program_counter: 0,
            stack: Stack::default(),
            memory: Memory::default(),
            calldata: Calldata::default(),
            storage: Storage::default(),
        }
    }

    pub fn new(
        bytecode: String,
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
            bytecode,
            calldata,
            command_length: 2,
            program_counter: 0,
            memory: defined_memory,
            storage: defined_storage,
            stack: defined_stack,
        }
    }

    pub fn get_bytecode(&self) -> &String {
        &self.bytecode
    }

    pub fn get_program_counter(&self) -> usize {
        self.program_counter
    }

    pub fn get_command_length(&self) -> usize {
        self.command_length
    }

    pub fn execute_opcode(&mut self, opcode: &str) -> Result<usize, ()> {
        let payload_size: usize;
        let i = self.program_counter + self.command_length;

        if opcode == "5f" {
            self.PUSH0();
            payload_size = 0;
        } else if opcode == "63" {
            payload_size = 8;
            let payload = self.bytecode[i..i + payload_size].to_string();

            self.PUSH4(&payload);
        } else if opcode == "73" {
            payload_size = 40;
            let payload = self.bytecode[i..i + payload_size].to_string();

            self.PUSH20(&payload);
        } else {
            payload_size = 0;
        }

        Ok(payload_size)
    }

    pub fn update_program_counter(&mut self, command: BytecodeExecutionTrail) {
        let current_opcode = self.bytecode
            [self.program_counter..self.program_counter + self.command_length]
            .to_string();

        match command {
            BytecodeExecutionTrail::Left => {
                self.program_counter =
                    self.program_counter.max(self.command_length) - self.command_length;
            }

            BytecodeExecutionTrail::Right => {
                let res = self.execute_opcode(&current_opcode);
                match res {
                    Ok(payload_size) => {
                        self.program_counter =
                            (self.program_counter + self.command_length + payload_size)
                                .min(self.bytecode.len() - self.command_length)
                    }
                    _ => (),
                }
            }
        }
    }
}

// 5f63ab12cd3473b2052eb9730b6afec7ae47b5bffe492c0c0e511b00

impl OPCODES for EVM {
    fn PUSH0(&mut self) {
        let _ = self.stack.push(&String::from("0"));
    }

    fn PUSH1(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH2(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH3(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH4(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH5(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH6(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH7(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH8(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH9(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH10(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH11(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH12(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH13(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH14(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH15(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH16(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH17(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH18(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH19(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH20(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH21(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH22(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH23(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH24(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH25(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH26(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH27(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH28(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH29(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH30(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH31(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }

    fn PUSH32(&mut self, value: &str) {
        let _ = self.stack.push(&value);
    }
}
