use crate::evm::{BytecodeExecutionTrail, EVM};
use ratatui::{
    crossterm::event::{self, Event::Key, KeyCode, KeyEventKind},
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Wrap},
    DefaultTerminal,
};
use std::io;

pub fn render(mut terminal: DefaultTerminal, execution_context: &mut EVM) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let i = execution_context.get_opcode_index();

            // known issues: INDEX OUT OF BOUNDS
            let display = Paragraph::new(Line::from(vec![
                Span::raw(&execution_context.get_bytecode()[..i]),
                Span::styled(
                    &execution_context.get_bytecode()[i..i + 2],
                    Style::new().bold().fg(Color::Green),
                ),
                Span::raw(&execution_context.get_bytecode()[(i + 2)..]),
            ]))
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
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Left | KeyCode::Up => {
                        execution_context.update_opcode_index(BytecodeExecutionTrail::Left(2))
                    }
                    KeyCode::Right | KeyCode::Down => {
                        execution_context.update_opcode_index(BytecodeExecutionTrail::Right(2))
                    }
                    _ => (),
                }
            }
        }
    }
}
