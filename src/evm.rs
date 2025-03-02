use crate::utils::{
    constants::Constants,
    operations::{
        convert_biguint_to_hex, convert_hex_to_biguint, convert_hex_to_dec, get_payload,
        pad_32_bytes,
    },
};
use crate::Calldata;
use crate::Memory;
use crate::Stack;
use crate::Storage;
use crate::OPCODES;
use num_bigint::BigUint;

pub enum ExecutionTrail {
    Left,
    Right,
}

#[derive(Debug)]
pub struct EVM {
    bytecode: String,
    program_counter: usize,
    command_length: usize,
    pub pc_history: Vec<usize>,

    pub stack: Stack,
    pub memory: Memory,
    pub calldata: Calldata,
    pub storage: Storage,
}

impl EVM {
    pub fn default(bytecode: String) -> EVM {
        EVM {
            bytecode,
            command_length: Constants::OPCODE_LENGTH(),
            program_counter: 0,
            pc_history: Vec::new(),
            stack: Stack::default(),
            memory: Memory::default(),
            calldata: Calldata::default(),
            storage: Storage::default(),
        }
    }

    pub fn new(bytecode: String, calldata: Calldata) -> EVM {
        EVM {
            bytecode,
            calldata,
            command_length: Constants::OPCODE_LENGTH(),
            program_counter: 0,
            pc_history: Vec::new(),
            memory: Memory::default(),
            storage: Storage::default(),
            stack: Stack::default(),
        }
    }
}

impl EVM {
    pub fn get_bytecode(&self) -> &String {
        &self.bytecode
    }

    pub fn get_program_counter(&self) -> usize {
        self.program_counter
    }

    pub fn get_command_length(&self) -> usize {
        self.command_length
    }
}

impl EVM {
    pub fn update_program_counter(&mut self, command: ExecutionTrail) {
        let current_opcode = self.bytecode
            [self.program_counter..self.program_counter + self.command_length]
            .to_string();

        match command {
            ExecutionTrail::Left => {
                let prev_index = self.pc_history.last();

                if let Some(value) = prev_index {
                    let _ = self.stack.undo();
                    self.program_counter = value.clone();
                    self.pc_history.pop();
                }
            }

            ExecutionTrail::Right => {
                let res = self.execute_opcode(&current_opcode);

                match res {
                    Ok(payload_size) => {
                        let pc_cache = (self.program_counter + self.command_length + payload_size)
                            .min(self.bytecode.len() - self.command_length);

                        if self.program_counter != pc_cache {
                            self.pc_history.push(self.program_counter);
                            self.program_counter = pc_cache;
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

impl EVM {
    pub fn execute_opcode(&mut self, opcode: &str) -> Result<usize, ()> {
        match opcode {
            "1c" => return Ok(self.SHR()),
            "35" => return Ok(self.CALLDATALOAD()),
            "50" => return Ok(self.POP()),
            "5f" => return Ok(self.PUSH0()),
            "60" => return Ok(self.PUSH1(&get_payload(&self, 1))),
            "61" => return Ok(self.PUSH2(&get_payload(&self, 2))),
            "62" => return Ok(self.PUSH3(&get_payload(&self, 3))),
            "63" => return Ok(self.PUSH4(&get_payload(&self, 4))),
            "64" => return Ok(self.PUSH5(&get_payload(&self, 5))),
            "65" => return Ok(self.PUSH6(&get_payload(&self, 6))),
            "66" => return Ok(self.PUSH7(&get_payload(&self, 7))),
            "67" => return Ok(self.PUSH8(&get_payload(&self, 8))),
            "68" => return Ok(self.PUSH9(&get_payload(&self, 9))),
            "69" => return Ok(self.PUSH10(&get_payload(&self, 10))),
            "6a" => return Ok(self.PUSH11(&get_payload(&self, 11))),
            "6b" => return Ok(self.PUSH12(&get_payload(&self, 12))),
            "6c" => return Ok(self.PUSH13(&get_payload(&self, 13))),
            "6d" => return Ok(self.PUSH14(&get_payload(&self, 14))),
            "6e" => return Ok(self.PUSH15(&get_payload(&self, 15))),
            "6f" => return Ok(self.PUSH16(&get_payload(&self, 16))),
            "70" => return Ok(self.PUSH17(&get_payload(&self, 17))),
            "71" => return Ok(self.PUSH18(&get_payload(&self, 18))),
            "72" => return Ok(self.PUSH19(&get_payload(&self, 19))),
            "73" => return Ok(self.PUSH20(&get_payload(&self, 20))),
            _ => Ok(0),
        }
    }
}

impl OPCODES for EVM {
    fn SHR(&mut self) -> usize {
        let shift_factor: usize;
        let value_to_shift: BigUint;
        let shifted_value: String;

        match self.stack.get_nth_element_from_top(0) {
            Some(x) => shift_factor = convert_hex_to_dec(&x),
            None => {
                // TODO: this should panic
                shift_factor = 0;
            }
        }

        match self.stack.get_nth_element_from_top(1) {
            Some(y) => value_to_shift = convert_hex_to_biguint(&y),
            None => {
                // TODO: this should panic
                value_to_shift = BigUint::ZERO;
            }
        }

        if shift_factor > u8::MAX as usize {
            shifted_value = String::from("0");
        } else {
            shifted_value = convert_biguint_to_hex(&(value_to_shift >> shift_factor));
        }
        let _ = self.stack.replace(2, vec![pad_32_bytes(&shifted_value)]);

        return 0;
    }

    fn CALLDATALOAD(&mut self) -> usize {
        if let Some(offset_from_stack) = self.stack.get_nth_element_from_top(0) {
            let _ = self.stack.replace(
                1,
                vec![self.calldata.read(convert_hex_to_dec(&offset_from_stack))],
            );
        } else {
            // TODO: ideally the program should crash
        }
        return 0;
    }

    fn POP(&mut self) -> usize {
        let _ = self.stack.pop();
        return 0;
    }

    fn PUSH0(&mut self) -> usize {
        let _ = self.stack.push(&String::from("00"));
        return 0;
    }

    fn PUSH1(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 1 * Constants::BYTE_SIZE();
    }

    fn PUSH2(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 2 * Constants::BYTE_SIZE();
    }

    fn PUSH3(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 3 * Constants::BYTE_SIZE();
    }

    fn PUSH4(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 4 * Constants::BYTE_SIZE();
    }

    fn PUSH5(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 5 * Constants::BYTE_SIZE();
    }

    fn PUSH6(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 6 * Constants::BYTE_SIZE();
    }

    fn PUSH7(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 7 * Constants::BYTE_SIZE();
    }

    fn PUSH8(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 8 * Constants::BYTE_SIZE();
    }

    fn PUSH9(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 9 * Constants::BYTE_SIZE();
    }

    fn PUSH10(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 10 * Constants::BYTE_SIZE();
    }

    fn PUSH11(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 11 * Constants::BYTE_SIZE();
    }

    fn PUSH12(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 12 * Constants::BYTE_SIZE();
    }

    fn PUSH13(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 13 * Constants::BYTE_SIZE();
    }

    fn PUSH14(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 14 * Constants::BYTE_SIZE();
    }

    fn PUSH15(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 15 * Constants::BYTE_SIZE();
    }

    fn PUSH16(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 16 * Constants::BYTE_SIZE();
    }

    fn PUSH17(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 17 * Constants::BYTE_SIZE();
    }

    fn PUSH18(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 18 * Constants::BYTE_SIZE();
    }

    fn PUSH19(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 19 * Constants::BYTE_SIZE();
    }

    fn PUSH20(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 20 * Constants::BYTE_SIZE();
    }

    fn PUSH21(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 21 * Constants::BYTE_SIZE();
    }

    fn PUSH22(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 22 * Constants::BYTE_SIZE();
    }

    fn PUSH23(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 23 * Constants::BYTE_SIZE();
    }

    fn PUSH24(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 24 * Constants::BYTE_SIZE();
    }

    fn PUSH25(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 25 * Constants::BYTE_SIZE();
    }

    fn PUSH26(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 26 * Constants::BYTE_SIZE();
    }

    fn PUSH27(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 27 * Constants::BYTE_SIZE();
    }

    fn PUSH28(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 28 * Constants::BYTE_SIZE();
    }

    fn PUSH29(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 29 * Constants::BYTE_SIZE();
    }

    fn PUSH30(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 30 * Constants::BYTE_SIZE();
    }

    fn PUSH31(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 31 * Constants::BYTE_SIZE();
    }

    fn PUSH32(&mut self, value: &str) -> usize {
        let _ = self.stack.push(&value);
        return 32 * Constants::BYTE_SIZE();
    }
}
