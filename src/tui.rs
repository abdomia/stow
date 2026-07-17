use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

const CHAT_PLACEHOLDER: &str = "Ask anything ...";

pub struct TextArea {
    text: String,
    cursor_position: usize,
    cursor_style: Style,
    placeholder_style: Style,
    text_style: Style,
}

impl Default for TextArea {
    fn default() -> Self {
        Self::new()
    }
}

impl TextArea {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor_position: 0,
            text_style: Style::default().fg(Color::Rgb(235, 238, 245)),
            placeholder_style: Style::default().fg(Color::Rgb(110, 118, 132)),
            cursor_style: Style::default()
                .fg(Color::Rgb(13, 17, 23))
                .bg(Color::Rgb(235, 238, 245)),
        }
    }

    pub fn input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.text.insert(self.cursor_index(), c);
                self.cursor_position += 1;
            }
            KeyCode::Backspace if self.cursor_position > 0 => {
                self.cursor_position -= 1;
                self.text.remove(self.cursor_index());
            }
            KeyCode::Delete if self.cursor_position < self.text_len() => {
                self.text.remove(self.cursor_index());
            }
            KeyCode::Left if self.cursor_position > 0 => self.cursor_position -= 1,
            KeyCode::Right if self.cursor_position < self.text_len() => self.cursor_position += 1,
            KeyCode::Home => self.cursor_position = 0,
            KeyCode::End => self.cursor_position = self.text_len(),
            _ => {}
        }
    }

    fn is_text_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn text_len(&self) -> usize {
        self.text.chars().count()
    }

    fn cursor_index(&self) -> usize {
        self.text
            .char_indices()
            .nth(self.cursor_position)
            .map(|(index, _)| index)
            .unwrap_or(self.text.len())
    }

    fn wrapped_lines(&self, width: usize) -> Vec<String> {
        if width == 0 {
            return vec![self.text.clone()];
        }

        textwrap::wrap(&self.text, width)
            .into_iter()
            .map(|cow| cow.into_owned())
            .collect()
    }

    fn cursor_cell(&self, width: usize) -> (usize, usize) {
        let mut chars_before = 0usize;
        let lines = self.wrapped_lines(width);
        for line in lines.iter() {
            let line_len = line.chars().count();
            if self.cursor_position <= chars_before + line_len {
                let col = self.cursor_position - chars_before;
                let row = self.wrapped_lines_rows_before(width, chars_before);
                return (row, col);
            }
            chars_before += line_len;
        }
        let rows = lines.len().saturating_sub(1);
        (rows, 0)
    }

    fn wrapped_lines_rows_before(&self, _width: usize, _chars_before: usize) -> usize {
        let lines = self.wrapped_lines(_width);
        let mut count = 0usize;
        let mut seen = 0usize;
        for line in &lines {
            if seen >= _chars_before {
                break;
            }
            seen += line.chars().count();
            count += 1;
        }
        count
    }
}

impl Widget for &TextArea {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        if self.is_text_empty() {
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

    let line = Line::from(vec![
        Span::styled(first_char.to_string(), cursor_style),
        Span::styled(rest, placeholder_style),
    ]);

    Paragraph::new(line)
        .style(placeholder_style)
        .render(area, buf);
}

fn render_text(area: Rect, buf: &mut Buffer, textarea: &TextArea) {
    let width = area.width as usize;
    let lines = textarea.wrapped_lines(width);
    let (cursor_row, cursor_col) = textarea.cursor_cell(width);

    let mut rendered_rows = Vec::with_capacity(lines.len());
    for (row, line) in lines.iter().enumerate() {
        let mut spans: Vec<Span> = Vec::with_capacity(line.chars().count().saturating_add(1));
        for (col, c) in line.chars().enumerate() {
            if row == cursor_row && col == cursor_col {
                spans.push(Span::styled(c.to_string(), textarea.cursor_style));
            } else {
                spans.push(Span::styled(c.to_string(), textarea.text_style));
            }
        }
        if row == cursor_row && cursor_col >= line.chars().count() {
            spans.push(Span::styled(" ", textarea.cursor_style));
        }
        rendered_rows.push(Line::from(spans));
    }

    Paragraph::new(rendered_rows)
        .style(textarea.text_style)
        .render(area, buf);
}

pub fn init_chat_ui() -> TextArea {
    TextArea::new()
}
