mod events;
mod stow;
mod tui;

use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{Terminal, backend::CrosstermBackend};
use stow::StowApp;

use crate::tui::{init_chat_ui, update_input};

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut stow_app = StowApp::new(terminal);
    stow_app.start()?;

    let (mut chat_input, area) = init_chat_ui();
    loop {
        stow_app.term.draw(|frame| {
            frame.render_widget(&chat_input, area);
        })?;

        if crossterm::event::poll(Duration::from_millis(150))? {
            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => break,
                        _ => update_input(&mut chat_input, key),
                    }
                }
            }
        }
    }
    stow_app.exit()?;
    Ok(())
}
