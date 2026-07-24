use crate::tui::TextArea;
use color_eyre::Result;
use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Stdout};
use std::panic;

pub type TerminalBackend = Terminal<CrosstermBackend<Stdout>>;

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

pub struct App {
    pub term: TerminalBackend,
}

pub struct AppFlow {
    pub prompt_area: TextArea,
    pub chat_scroll: i32,
    pub messages: Vec<Message>,
}

impl AppFlow {
    pub fn init_flow() -> Self {
        Self {
            chat_scroll: 0,
            prompt_area: TextArea::new(),
            messages: Vec::new(),
        }
    }

    pub fn send_message(&mut self) {
        let text = self.prompt_area.text().trim().to_string();
        if text.is_empty() {
            return;
        }
        self.messages.push(Message {
            role: Role::User,
            content: text,
        });
        self.prompt_area.clear();
    }

    pub fn clamp_scroll(&mut self, view_height: u16) {
        let content_h = self.content_height() as i32;
        let max_scroll = content_h.saturating_sub(view_height as i32).max(0);
        self.chat_scroll = self.chat_scroll.min(max_scroll);
    }

    pub fn scroll_chat(&mut self, delta: i32) {
        let max_scroll = (self.content_height() as i32).saturating_sub(1).max(0);
        let new = self.chat_scroll + delta;
        self.chat_scroll = new.clamp(0, max_scroll);
    }

    fn content_height(&self) -> usize {
        self.messages
            .iter()
            .map(|m| 2 + m.content.lines().count())
            .sum()
    }
}

impl App {
    pub fn new(term: TerminalBackend) -> Self {
        Self { term }
    }

    pub fn start(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;

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
        execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.term.show_cursor()?;
        Ok(())
    }
}
