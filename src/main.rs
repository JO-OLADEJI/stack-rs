mod evm;
mod structs;
mod tui;
mod utils;

use evm::EVM;
use huff_core::Compiler;
use huff_utils::evm_version::EVMVersion;
use std::{
    io::{self, Write},
    process,
    sync::Arc,
};
use structs::{
    calldata::Calldata, memory::Memory, opcodes::OPCODES, stack::Stack, storage::Storage,
};
use tui::render;

fn main() {
    let evm_version = EVMVersion::default();
    let mut calldata = String::new();
    let mut compiled_bytecode = String::new();

    let compiler = Compiler::new(
        &evm_version,
        Arc::new(vec!["./playground.huff".to_string()]),
        None,
        None,
        None,
        None,
        None,
        false,
        false,
    );

    let result = compiler.execute();
    match result {
        Ok(artifact) => {
            compiled_bytecode.push_str(&artifact[0].runtime[..]);
        }

        Err(error) => {
            println!("Error: {:?}", error);
            process::exit(1)
        }
    }

    print!("calldata: ");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut calldata);

    calldata = calldata.trim().to_string();

    let mut terminal = ratatui::init();
    let __ = terminal.clear();
    let mut execution_context = EVM::new(compiled_bytecode, Calldata::new(&calldata));
    let ___ = render::render(terminal, &mut execution_context);
    ratatui::restore();
}
