mod evm;
mod structs;
mod tui;
mod utils;

use evm::EVM;
use huff_core::Compiler;
use huff_utils::evm_version::EVMVersion;
use std::{
    env,
    io::{self, Result, Write},
    process,
    sync::Arc,
};
use structs::{
    calldata::Calldata, memory::Memory, opcodes::OPCODES, stack::Stack, storage::Storage,
};
use tui::render;

fn main() -> Result<()> {
    let evm_version = EVMVersion::default();
    let mut calldata = String::new();
    let mut compiled_bytecode = String::new();

    print!("calldata: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut calldata)?;

    calldata = calldata.trim().to_string();

    let compiler = Compiler::new(
        &evm_version,                                    // EVM version
        Arc::new(vec!["./playground.huff".to_string()]), // sources - file paths
        None,                                            // output - file path
        None,                                            // alternative `MAIN()` macro
        None,                                            // alternative `CONSTRUCTOR()` macro
        None,  // constructor arguments - for the generation of runtime code
        None,  // constant overrides
        false, // verbose ??
        false, // check cached artifacts ??
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

    match Calldata::new(&calldata) {
        Ok(instance) => {
            let mut terminal = ratatui::init();
            terminal.clear()?;
            let mut execution_context = EVM::new(compiled_bytecode, instance, None, None, None);
            let __ = render::render(terminal, &mut execution_context);
            ratatui::restore();
        }
        Err(reason) => {
            println!("{}", reason);
            process::exit(1)
        }
    }

    Ok(())
}
