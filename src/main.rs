mod evm;
mod structs;
mod tui;
mod utils;

use evm::EVM;
use structs::{
    calldata::Calldata, memory::Memory, opcodes::OPCODES, stack::Stack, storage::Storage,
};
use tui::render;

use huff_core::Compiler;
use huff_utils::evm_version::EVMVersion;
use std::{
    io::{self, Write},
    sync::Arc,
};

fn main() -> io::Result<()> {
    let mut file_path = String::new();
    let evm_version = EVMVersion::default();
    let mut sources: Vec<String> = Vec::new();
    let mut compiled_bytecode = String::new();

    print!("source: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut file_path)
        .expect("error: file read");
    println!("file path: {}", file_path);
    sources.push(file_path);

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
        }
    }

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut execution_context = EVM::default(compiled_bytecode);
    let __ = render::render(terminal, &mut execution_context);

    ratatui::restore();
    __
}
