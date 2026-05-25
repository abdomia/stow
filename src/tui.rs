use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders};
use tui_textarea::TextArea;

pub fn init_chat_ui() -> (TextArea<'static>, Rect) {
    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .title(" Chat ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    textarea.set_cursor_line_style(Style::default().fg(Color::LightMagenta));
    textarea.set_placeholder_text("ask what do you want!");
    textarea.set_placeholder_style(Style::default().dim());

    let chat_area = Rect {
        x: 5,
        y: 5,
        width: 40,
        height: 5,
    };

    (textarea, chat_area)
}

pub fn update_input(text_input: &mut TextArea<'static>, keys: KeyEvent) {
    text_input.input(keys);
}
