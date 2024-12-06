use huff_core::Compiler;
use huff_utils::evm_version::EVMVersion;
use ratatui::{
    crossterm::event::{self, Event::Key, KeyCode, KeyEventKind},
    layout::Alignment,
    style::{Style, Stylize},
    widgets::{Block, Padding, Paragraph, Wrap},
    DefaultTerminal,
};
use std::{
    io::{self, Write},
    sync::Arc,
};

fn main() -> io::Result<()> {
    let mut sources: Vec<String> = Vec::new();
    let mut file_path = String::new();
    let evm_version = EVMVersion::default();
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
            compiled_bytecode.push_str(&artifact[0].bytecode[..]);
        }

        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    let mut terminal = ratatui::init();
    terminal.clear()?;
    let __ = render(terminal, &compiled_bytecode);

    ratatui::restore();
    __
}

fn render(mut terminal: DefaultTerminal, bytecode: &String) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let display = Paragraph::new(&bytecode[..])
                .block(
                    Block::bordered()
                        .title("  bytecode ")
                        .title_alignment(Alignment::Center)
                        .title_style(Style::new().bold())
                        .padding(Padding::horizontal(2)),
                )
                .wrap(Wrap { trim: true })
                .white();
            frame.render_widget(display, frame.area());
        })?;

        if let Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
