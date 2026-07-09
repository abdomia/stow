use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

const CHAT_PLACEHOLDER: &str = "Ask anything";

pub struct TextArea {
    text: String,
    cursor: usize,
    cursor_style: Style,
    placeholder_style: Style,
    text_style: Style,
}

impl TextArea {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            text_style: Style::default().fg(Color::White),
            placeholder_style: Style::default().fg(Color::DarkGray),
            cursor_style: Style::default().fg(Color::Black).bg(Color::White),
        }
    }

    pub fn input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.text.insert(self.cursor_index(), c);
                self.cursor += 1;
            }
            KeyCode::Backspace if self.cursor > 0 => {
                self.cursor -= 1;
                self.text.remove(self.cursor_index());
            }
            KeyCode::Delete if self.cursor < self.len() => {
                self.text.remove(self.cursor_index());
            }
            KeyCode::Left if self.cursor > 0 => self.cursor -= 1,
            KeyCode::Right if self.cursor < self.len() => self.cursor += 1,
            KeyCode::Home => self.cursor = 0,
            KeyCode::End => self.cursor = self.len(),
            _ => {}
        }
    }

    fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn len(&self) -> usize {
        self.text.chars().count()
    }

    fn cursor_index(&self) -> usize {
        self.text
            .char_indices()
            .nth(self.cursor)
            .map(|(index, _)| index)
            .unwrap_or(self.text.len())
    }
}

impl Widget for &TextArea {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        if self.is_empty() {
            render_placeholder(area, buf, self.placeholder_style, self.cursor_style);
            return;
        }

        render_text(area, buf, self);
    }
}

fn render_placeholder(area: Rect, buf: &mut Buffer, placeholder_style: Style, cursor_style: Style) {
    let Some(first_char) = CHAT_PLACEHOLDER.chars().next() else {
        return;
    };
    let rest = &CHAT_PLACEHOLDER[first_char.len_utf8()..];
    let rest_width = area.width.saturating_sub(1) as usize;

    let line = Line::from(vec![
        Span::styled(first_char.to_string(), cursor_style),
        Span::styled(
            rest.chars().take(rest_width).collect::<String>(),
            placeholder_style,
        ),
    ]);

    Paragraph::new(line)
        .style(placeholder_style)
        .render(area, buf);
}

fn render_text(area: Rect, buf: &mut Buffer, textarea: &TextArea) {
    let width = area.width as usize;
    let text_len = textarea.len();
    let start = textarea.cursor.saturating_sub(width.saturating_sub(1));
    let visible_chars: Vec<char> = textarea.text.chars().skip(start).take(width).collect();
    let cursor_col = textarea.cursor.saturating_sub(start);

    let mut spans = Vec::with_capacity(visible_chars.len() + 1);
    for (col, c) in visible_chars.iter().enumerate() {
        let style = if col == cursor_col {
            textarea.cursor_style
        } else {
            textarea.text_style
        };
        spans.push(Span::styled(c.to_string(), style));
    }

    if textarea.cursor == text_len && cursor_col < width {
        spans.push(Span::styled(" ", textarea.cursor_style));
    }

    Paragraph::new(Line::from(spans))
        .style(textarea.text_style)
        .render(area, buf);
}

pub fn init_chat_ui() -> (TextArea, Rect) {
    let textarea = TextArea::new();

    let chat_area = Rect {
        x: 3,
        y: Position::default().y,
        width: 40,
        height: 1,
    };

    (textarea, chat_area)
}
