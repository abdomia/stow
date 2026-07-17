mod events;
mod stow;
mod tui;

use crate::events::update_input;
use crate::stow::StowApp;
use crate::tui::init_chat_ui;
use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{Terminal, backend::CrosstermBackend, layout::Rect};

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut stow_app = StowApp::new(terminal);
    stow_app.start()?;

    let mut chat_input = init_chat_ui();
    loop {
        stow_app.term.draw(|frame| {
            let area = frame.area();
            let chat_area = Rect {
                x: 3,
                y: area.height.saturating_sub(8),
                width: area.width.saturating_sub(6),
                height: 6,
            };
            frame.render_widget(&chat_input, chat_area);
        })?;

        if let Event::Key(key) = crossterm::event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => break,
                    _ => update_input(&mut chat_input, key),
                }
            }
        }
    }

    stow_app.exit()?;
    Ok(())
}
