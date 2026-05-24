use color_eyre::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use ratatui::backend::CrosstermBackend;

use std::io::{self, Stderr};
use std::panic;
use tui_textarea::TextArea;

type Term = ratatui::Terminal<CrosstermBackend<Stderr>>;
pub struct StowApp {
    pub input: TextArea<'static>,
    pub term: Term,
}

impl StowApp {
    pub fn new(term: Term) -> Self {
        Self {
            term,
            input: TextArea::default(),
        }
    }

    pub fn start(&mut self) -> std::io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.term.hide_cursor()?;
        self.term.clear()?;
        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.term.show_cursor()?;
        Ok(())
    }
}
