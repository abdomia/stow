mod app;
mod input;
mod intro_anim;
mod phases;
mod tui;

use std::time::{Duration, Instant};

use crate::app::{App, AppFlow};
use crate::input::keymap::{Action, handle_key};
use crate::intro_anim::{LogoAnim, frame_to_text};
use crate::phases::{AppPhase, updated_phase};
use crate::tui::layout::{ACCENT, ACCENT_SOFT};

use color_eyre::Result;
use crossterm::event::{Event, KeyEventKind};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::style::Style;

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut stow_app = App::new(terminal);
    let mut app_flow = AppFlow::init_flow();
    let mut phase = AppPhase::Splash(Instant::now());

    let anim = LogoAnim::new().expect("failed to decode stow.gif");
    let (w, h, term_h) = anim.dim();

    stow_app.start()?;
    loop {
        let timeout = Duration::from_millis(16);
        if crossterm::event::poll(timeout)? {
            match crossterm::event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if !matches!(phase, AppPhase::Normal) {
                        phase = AppPhase::Normal;
                        continue;
                    }
                    match handle_key(&mut app_flow, key) {
                        Action::Quit => break,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        phase = updated_phase(phase.clone(), &anim);

        // handle rendering of widgets across the application.
        stow_app.term.draw(|frame| {
            let area = frame.area();
            frame.buffer_mut().set_style(
                area,
                Style::default().bg(tui::layout::BG).fg(tui::layout::FG),
            );

            AppPhase::display_phase(&phase, frame, &area, w, h, term_h, &mut app_flow, &anim);
        })?;
    }

    stow_app.exit()?;
    Ok(())
}
