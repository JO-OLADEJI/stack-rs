use crate::evm::{BytecodeExecutionTrail, EVM};
use ratatui::{
    crossterm::event::{self, Event::Key, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListDirection, Padding, Paragraph, Wrap},
    DefaultTerminal,
};
use std::io;

pub fn render(mut terminal: DefaultTerminal, execution_context: &mut EVM) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let h_grid = Layout::horizontal(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
                .split(frame.area());
            let v_grid =
                Layout::vertical(vec![Constraint::Length(3), Constraint::Min(0)]).split(h_grid[1]);

            let i = execution_context.get_program_counter();
            let n = execution_context.get_command_length();

            // known issues: INDEX OUT OF BOUNDS
            let display = Paragraph::new(Line::from(vec![
                Span::raw(&execution_context.get_bytecode()[..i]),
                Span::styled(
                    &execution_context.get_bytecode()[i..i + n],
                    Style::new().bold().fg(Color::Green),
                ),
                Span::raw(&execution_context.get_bytecode()[(i + n)..]),
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

            // render bytecode
            frame.render_widget(display, h_grid[0]);

            let pc = Paragraph::new(execution_context.get_program_counter().to_string()).block(
                Block::bordered()
                    .title("  PC ")
                    .padding(Padding::horizontal(2)),
            );

            // render program counter
            frame.render_widget(pc, v_grid[0]);

            let list = List::new(execution_context.stack.trace.clone())
                .block(
                    Block::bordered()
                        .title("  Stack ")
                        .padding(Padding::horizontal(2)),
                )
                .direction(ListDirection::BottomToTop);

            // render stack
            frame.render_widget(list, v_grid[1]);
        })?;

        if let Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Left | KeyCode::Up => {
                        execution_context.update_program_counter(BytecodeExecutionTrail::Left)
                    }
                    KeyCode::Right | KeyCode::Down => {
                        execution_context.update_program_counter(BytecodeExecutionTrail::Right)
                    }
                    _ => (),
                }
            }
        }
    }
}
