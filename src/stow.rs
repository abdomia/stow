use color_eyre::Result;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{self, LeaveAlternateScreen},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Stdout};
use std::panic;

type Term = Terminal<CrosstermBackend<Stdout>>;

pub struct StowApp {
    pub term: Term,
}

impl StowApp {
    pub fn new(term: Term) -> Self {
        Self { term }
    }

    pub fn start(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        // execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

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
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.term.show_cursor()?;
        Ok(())
    }
}
