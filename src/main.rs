use huff_core::Compiler;
use huff_utils::evm_version::EVMVersion;
use std::{
    io::{self, Write},
    sync::Arc,
};

fn main() {
    let mut sources: Vec<String> = Vec::new();
    let evm_version = EVMVersion::default();
    let mut file_path = String::new();

    print!("source: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut file_path)
        .expect("error: file read");
    println!("file path: {}", file_path);

    sources.push(file_path);

    // vec!["./playground.huff".to_string()]
    let compiler = Compiler::new(
        &evm_version,      // EVM version
        Arc::new(sources), // sources - file paths
        None,              // output - file path
        None,              // alternative `MAIN()` macro
        None,              // alternative `CONSTRUCTOR()` macro
        None,              // constructor arguments - for the generation of runtime code
        None,              // constant overrides
        false,             // verbose ??
        false,             // check cached artifacts ??
    );

    let result = compiler.execute();
    match result {
        Ok(artifact) => {
            println!("bytecode: {}", artifact[0].bytecode);
        }

        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
