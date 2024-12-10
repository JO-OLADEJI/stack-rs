pub trait OPCODES {
    // /// Halts execution and exits the current context successfully
    // fn STOP(&mut self);

    // /// Addition operation
    // /// Stack input - a, b]
    // /// stack output - a + b]
    // fn ADD(&mut self);

    // /// Multiplication operation
    // /// Stack input - a, b]
    // /// stack output - a * b]
    // fn MUL(&mut self);

    // /// Subtraction operation
    // /// Stack input - a, b]
    // /// stack output - a - b]
    // fn SUB(&mut self);

    // /// Integer division operation
    // /// Stack input - a, b]
    // /// stack output - a // b]
    // fn DIV(&mut self);

    // /// Signed integer division operation (truncated)
    // /// Stack input - a, b]
    // /// stack output - a // b]
    // fn SDIV(&mut self);

    // /// Modulo remainder operation
    // /// Stack input - a, b]
    // /// stack output - a % b]
    // fn MOD(&mut self);

    // /// Signed modulo remainder operation
    // /// Stack input - a, b]
    // /// stack output - a % b]
    // fn SMOD(&mut self);

    // /// Modulo addition operation
    // /// Stack input - a, b, n]
    // /// stack output - (a + b) % n]
    // fn ADDMOD(&mut self);

    // /// Modulo multiplication operation
    // /// Stack input - a, b, n]
    // /// stack output - (a * b) % n]
    // fn MULMOD(&mut self);

    // /// Exponential operation
    // /// Stack input - a, exponent]
    // /// stack output - a ** exponent]
    // fn EXP(&mut self);

    // /// Extend length of two's complement signed integer
    // /// Stack input - b, x]
    // /// stack output - y]
    // fn SIGNEXTEND(&mut self);

    // /// Less-than comparison
    // /// Stack input - a, b]
    // /// stack output - a < b]
    // fn LT(&mut self);

    // /// Greater-than comparison
    // /// Stack input - a, b]
    // /// stack output - a > b]
    // fn GT(&mut self);

    // /// Signed less-than comparison
    // /// Stack input - a, b]
    // /// stack output - a < b]
    // fn SLT(&mut self);

    // /// Signed greater-than comparison
    // /// Stack input - a, b]
    // /// stack output - a > b]
    // fn SGT(&mut self);

    // /// Equality comparison
    // /// Stack input - a, b]
    // /// stack output - a == b]
    // fn EQ(&mut self);

    // /// Is-zero comparison
    // /// Stack input - a]
    // /// stack output - a == 0]
    // fn ISZERO(&mut self);

    // /// Bitwise AND operation
    // /// Stack input - a, b]
    // /// stack output - a & b]
    // fn AND(&mut self);

    // /// Bitwise OR operation
    // /// Stack input - a, b]
    // /// stack output - a | b]
    // fn OR(&mut self);

    // /// Bitwise XOR operation
    // /// Stack input - a, b]
    // /// stack output - a ^ b]
    // fn XOR(&mut self);

    // /// Bitwise NOT operation
    // /// Stack input - a]
    // /// stack output - ~a]
    // fn NOT(&mut self);

    // /// Retrieve single byte from word
    // /// Stack input - i, x]
    // /// stack output - y]
    // fn BYTE(&mut self);

    // /// Left shift operation
    // /// Stack input - shift, value]
    // /// stack output - value << shift]
    // fn SHL(&mut self);

    // /// Right shift operation
    // /// Stack input - shift, value]
    // /// stack output - value >> shift]
    // fn SHR(&mut self);

    // /// Arithmetic (signed) right shift operation
    // /// Stack input - shift, value]
    // /// stack output - value >> shift]
    // fn SAR(&mut self);

    // /// Compute Keccak-256 hash
    // /// Stack input - offset, size]
    // /// stack output - hash]
    // fn KECCAK256(&mut self);

    // /// Get address of currently executing account
    // /// stack input - []
    // /// stack output - hash]
    // fn ADDRESS(&mut self);

    // /// Get balance of the given account
    // /// stack input - address]
    // /// stack output - balance]
    // fn BALANCE(&mut self);

    // /// Get execution origination address
    // /// stack input - []
    // /// stack output - address]
    // fn ORIGIN(&mut self);

    // /// Get calleer address
    // /// stack input - []
    // /// stack output - address]
    // fn CALLER(&mut self);

    // /// Get deposited value by the instruction/transaction responsible for this execution
    // /// stack input - []
    // /// stack output - value]
    // fn CALLVALUE(&mut self);

    // /// Get input data of current environment
    // /// stack input - i]
    // /// stack output - data[i:i+32]]
    // fn CALLDATALOAD(&mut self);

    // /// Get size of input data in current environment
    // /// stack input - []
    // /// stack output - size]
    // fn CALLDATASIZE(&mut self);

    // /// Copy input data in current environment to memory
    // /// stack input - destOffset, offset, size]
    // /// stack output - []
    // fn CALLDATACOPY(&mut self);

    // /// Get size of code running in current environment
    // /// stack input - []
    // /// stack output - size]
    // fn CODESIZE(&mut self);

    // /// Copy code running in current environment to memory
    // /// stack input - destOffset, offset, size]
    // /// stack output - []
    // fn CODECOPY(&mut self);

    // /// Get price of gas in current environment
    // /// stack input - []
    // /// stack output - price]
    // fn GASPRICE(&mut self);

    // /// Get size of an account's code
    // /// stack input - address]
    // /// stack output - size]
    // fn EXTCODESIZE(&mut self);

    // /// Copy an account's code to memory
    // /// stack input - address, destOffset, offset, size]
    // /// stack output - []
    // fn EXTCODECOPY(&mut self);

    // /// Get size of output data fro the previous call from the current environment
    // /// stack input - []
    // /// stack output - size]
    // fn RETURNDATASIZE(&mut self);

    // /// Copy output data from the previous call to memory
    // /// stack input - destOffset, offset, size]
    // /// stack output - []
    // fn RETURNDATACOPY(&mut self);

    // /// Get hash of an account's code
    // /// stack input - address]
    // /// stack output - hash]
    // fn EXTCODEHASH(&mut self);

    // /// Get the hash of one of the 256 most recent complete blocks
    // /// stack input - blockNumber]
    // /// stack output - hash]
    // fn BLOCKHASH(&mut self);

    // /// Get the block's beneficiary address
    // /// stack input - []
    // /// stack output - address]
    // fn COINBASE(&mut self);

    // /// Get the block's timestamp
    // /// stack input - []
    // /// stack output - timestamp]
    // fn TIMESTAMP(&mut self);

    // /// Get the block's number
    // /// stack input - []
    // /// stack output - blockNumber]
    // fn NUMBER(&mut self);

    // /// Get the block's difficulty
    // /// stack input - []
    // /// stack output - difficulty]
    // fn PREVRANDAO(&mut self);

    // /// Get the block's gas limit
    // /// stack input - []
    // /// stack output - gasLimit]
    // fn GASLIMIT(&mut self);

    // /// Get the chain ID
    // /// stack input - []
    // /// stack output - chainId]
    // fn CHAINID(&mut self);

    // /// Get the balance of currently executing account
    // /// stack input - []
    // /// stack output balance]
    // fn SELFBALANCE(&mut self);

    // /// Get the base fee
    // /// stack input - []
    // /// stack output - baseFee]
    // fn BASEFEE(&mut self);

    // /// Get versioned hashes
    // /// stack input - index]
    // /// stack output - blobVersionedHashesAtIndex]
    // fn BLOBHASH(&mut self);

    // /// Returns the value of the blob base-fee of the current block
    // /// stack input - []
    // /// stack output - blobBaseFee]
    // fn BLOBBASEFEE(&mut self);

    // /// Remove item from stack
    // /// stack input - y]
    // /// stack output - []
    // fn POP(&mut self);

    // /// Load word from memory
    // /// stack input - offset]
    // /// stack output - value]
    // fn MLOAD(&mut self);

    // /// Save word to memory
    // /// stack input - offset, value]
    // /// stack output - []
    // fn MSTORE(&mut self);

    // /// Save byte to memory
    // /// stack input - offset, value]
    // /// stack output - []
    // fn MSTORE8(&mut self);

    // /// Load word from storage
    // /// stack input - key]
    // /// stack output - value]
    // fn SLOAD(&mut self);

    // /// Save word to storage
    // /// stack input - key, value]
    // /// stack output - []
    // fn SSTORE(&mut self);

    // /// Alter the program counter
    // /// stack input - counter]
    // /// stack output - []
    // fn JUMP(&mut self);

    // /// Conditionally alter the program counter
    // /// stack input - counter, b]
    // /// stack output - []
    // fn JUMPI(&mut self);

    // /// Get the value of the program counter prior to the increment corresponding to this instruction
    // /// stack input - []
    // /// stack output - counter]
    // fn PC(&mut self);

    // /// Get the size of active memory in bytes
    // /// stack input - []
    // /// stack output - size]
    // fn MSIZE(&mut self);

    // /// Get the amount of available gas, including the corresponding reduction for the cost of this instruction
    // /// stack input - []
    // /// stack output - gas]
    // fn GAS(&mut self);

    // /// Mark a valid destination for jumps
    // /// stack input - []
    // /// stack output - []
    // fn JUMPDEST(&mut self);

    // /// Load word from transient storage
    // /// stack input - key]
    // /// stack output - value]
    // fn TLOAD(&mut self);

    // /// Save word to transient storage
    // /// stack input - key, value]
    // /// stack output - []
    // fn TSTORE(&mut self);

    // /// Copy memory areas
    // /// stack input - destOffset, offset, size]
    // /// stack output []
    // fn MCOPY(&mut self);

    /// Place 0 on stack
    /// stack input - []
    /// stack output - 0]
    fn PUSH0(&mut self);

    /// Place 1 byte item on stack
    /// stack input - []
    /// stack output - value]
    fn PUSH1(&mut self, value: &str);

    /// Place 2 byte item on stack
    /// stack input - []
    /// stack output - value]
    fn PUSH2(&mut self, value: &str);

    /// Place 3 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH3(&mut self, value: &str);

    /// Place 4 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH4(&mut self, value: &str);

    /// Place 5 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH5(&mut self, value: &str);

    /// Place 6 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH6(&mut self, value: &str);

    /// Place 7 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH7(&mut self, value: &str);

    /// Place 8 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH8(&mut self, value: &str);

    /// Place 9 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH9(&mut self, value: &str);

    /// Place 10 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH10(&mut self, value: &str);

    /// Place 11 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH11(&mut self, value: &str);

    /// Place 12 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH12(&mut self, value: &str);

    /// Place 13 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH13(&mut self, value: &str);

    /// Place 14 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH14(&mut self, value: &str);

    /// Place 15 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH15(&mut self, value: &str);

    /// Place 16 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH16(&mut self, value: &str);

    /// Place 17 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH17(&mut self, value: &str);

    /// Place 18 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH18(&mut self, value: &str);

    /// Place 19 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH19(&mut self, value: &str);

    /// Place 20 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH20(&mut self, value: &str);

    /// Place 221 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH21(&mut self, value: &str);

    /// Place 22 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH22(&mut self, value: &str);

    /// Place 23 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH23(&mut self, value: &str);

    /// Place 24 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH24(&mut self, value: &str);

    /// Place 25 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH25(&mut self, value: &str);

    /// Place 26 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH26(&mut self, value: &str);

    /// Place 27 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH27(&mut self, value: &str);

    /// Place 28 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH28(&mut self, value: &str);

    /// Place 29 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH29(&mut self, value: &str);

    /// Place 30 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH30(&mut self, value: &str);

    /// Place 31 byte item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH31(&mut self, value: &str);

    /// Place 32 byte (full word) item on stack
    /// stack input - []
    /// stack output value]
    fn PUSH32(&mut self, value: &str);

    // /// Duplicate stack item
    // /// stack input - value]
    // /// stack output - value, value]
    // fn DUP1(&mut self);

    // /// Duplicate 2nd stack item
    // /// stack input - a, b]
    // /// stack output - b, a, b]
    // fn DUP2(&mut self);

    // /// Duplicate 3rd stack item
    // /// stack input - a, b, c]
    // /// stack output - c, a, b, c]
    // fn DUP3(&mut self);

    // /// Duplicate 4th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP4(&mut self);

    // /// Duplicate 5th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP5(&mut self);

    // /// Duplicate 6th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP6(&mut self);

    // /// Duplicate 7th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP7(&mut self);

    // /// Duplicate 8th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP8(&mut self);

    // /// Duplicate 9th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP9(&mut self);

    // /// Duplicate 10th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP10(&mut self);

    // /// Duplicate 11th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP11(&mut self);

    // /// Duplicate 12th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP12(&mut self);

    // /// Duplicate 13th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP13(&mut self);

    // /// Duplicate 14th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP14(&mut self);

    // /// Duplicate 15th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP15(&mut self);

    // /// Duplicate 16th stack item
    // /// stack input - ..., value]
    // /// stack output - value, ..., value]
    // fn DUP16(&mut self);

    // /// Exchange 1st and 2nd stack items
    // /// stack input - a, b]
    // /// stack output - b, a]
    // fn SWAP1(&mut self);

    // /// Exchange 1st and 3nd stack items
    // /// stack input - a, b, c]
    // /// stack output - c, b, a]
    // fn SWAP2(&mut self);

    // /// Exchange 1st and 4th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP3(&mut self);

    // /// Exchange 1st and 5th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP4(&mut self);

    // /// Exchange 1st and 6th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP5(&mut self);

    // /// Exchange 1st and 7th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP6(&mut self);

    // /// Exchange 1st and 8th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP7(&mut self);

    // /// Exchange 1st and 9th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP8(&mut self);

    // /// Exchange 1st and 10th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP9(&mut self);

    // /// Exchange 1st and 11th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP10(&mut self);

    // /// Exchange 1st and 12th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP11(&mut self);

    // /// Exchange 1st and 13th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP12(&mut self);

    // /// Exchange 1st and 14th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP13(&mut self);

    // /// Exchange 1st and 15th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP14(&mut self);

    // /// Exchange 1st and 16th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP15(&mut self);

    // /// Exchange 1st and 17th stack items
    // /// stack input - a, ..., b]
    // /// stack output - b, ..., a]
    // fn SWAP16(&mut self);

    // /// Append log record with no topics
    // /// stack input - offset, size]
    // /// stack output - []
    // fn LOG0(&mut self);

    // /// Append log record with 1 topics
    // /// stack input - offset, size, topic]
    // /// stack output - []
    // fn LOG1(&mut self);

    // /// Append log record with 2 topics
    // /// stack input - offset, size, topic1, topic2]
    // /// stack output - []
    // fn LOG2(&mut self);

    // /// Append log record with 3 topics
    // /// stack input - offset, size, topic1, topic2, topic3]
    // /// stack output - []
    // fn LOG3(&mut self);

    // /// Append log record with 4 topics
    // /// stack input - offset, size, topic1, topic2, topic3, topic4]
    // /// stack output - []
    // fn LOG4(&mut self);

    // /// Create a new account with associated code
    // /// stack input - value, offset, size]
    // /// stack output - address]
    // fn CREATE(&mut self);

    // /// Message-call into an account
    // /// stack input - gas, address, value, argsOffset, argsSize, retOffset, retSize]
    // /// stack output - success]
    // fn CALL(&mut self);

    // /// Message-call into an account with alternative account's code
    // /// stack input - gas, address, value, argsOffset, argsSize, retOffset, retSize]
    // /// stack output - success]
    // fn CALLCODE(&mut self);

    // /// Halt execution returning output data
    // /// stack input - offset, size]
    // /// stack output - []
    // fn RETURN(&mut self);

    // /// Message-call into this account with an alternative account's code, but persisting the current values for sender and value
    // /// stack input - gas, address, argsOffset, argsSize, retOffset, retSize]
    // /// stack output -  success]
    // fn DELEGATECALL(&mut self);

    // /// Create a new account with associated code at a predictable address
    // /// stack input - value, offset, size, salt]
    // /// stack output - address]
    // fn CREATE2(&mut self);

    // /// Static message-call into an account
    // /// stack input - gas, address, argsOffset, argsSize, retOffset, retSize]
    // /// stack output - success]
    // fn STATICCALL(&mut self);

    // /// Halt execution reverting state changes but returning data and remaining gas
    // /// stack input - offset, size]
    // /// stack output - []
    // fn REVERT(&mut self);

    // /// Designated invalid instruction
    // /// stack input - []
    // /// stack outpout - []
    // fn INVALID(&mut self);

    // /// Halt execution and register account for later deletion or send all ether to address (post-cancun)
    // /// stack input - address]
    // /// stack outpout - []
    // fn SELFDESTRUCT(&mut self);
}

// pub trait OPCODES {

// }
