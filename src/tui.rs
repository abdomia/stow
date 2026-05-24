use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders};
use tui_textarea::TextArea;

pub fn chat_ui(f: &mut Frame) {
    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .title(" Chat ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    textarea.set_cursor_line_style(Style::default().fg(Color::LightMagenta));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(4),
            Constraint::Length(1),
        ])
        .split(f.area());

    f.render_widget(&textarea, layout[0]);
}
