mod events;
mod stow;
mod tui;

use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{Terminal, backend::CrosstermBackend};
use stow::StowApp;

use crate::tui::chat_ui;

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut stow_tui = StowApp::new(terminal);
    stow_tui.start()?;

    loop {
        stow_tui.term.draw(|frame| {
            chat_ui(frame);
        })?;

        if crossterm::event::poll(Duration::from_millis(150))? {
            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => break,
                        _ => todo!(),
                    }
                }
            }
        }
    }
    stow_tui.exit()?;
    Ok(())
}
