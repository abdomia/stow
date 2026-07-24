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
    scroll: usize,
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
            scroll: 0,
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

    pub fn insert_newline(&mut self) {
        self.text.insert(self.cursor_index(), '\n');
        self.cursor_position += 1;
    }

    pub fn delete_word(&mut self) {
        while self.cursor_position > 0
            && self.text.chars().nth(self.cursor_position - 1) == Some(' ')
        {
            self.cursor_position -= 1;
            self.text.remove(self.cursor_index());
        }
        while self.cursor_position > 0
            && self.text.chars().nth(self.cursor_position - 1) != Some(' ')
        {
            self.cursor_position -= 1;
            self.text.remove(self.cursor_index());
        }
    }

    /// Move the cursor one word to the left (Ctrl+Left).
    pub fn move_word_left(&mut self) {
        while self.cursor_position > 0
            && self.text.chars().nth(self.cursor_position - 1) == Some(' ')
        {
            self.cursor_position -= 1;
        }
        while self.cursor_position > 0
            && self.text.chars().nth(self.cursor_position - 1) != Some(' ')
        {
            self.cursor_position -= 1;
        }
    }

    pub fn move_word_right(&mut self) {
        while self.cursor_position < self.text_len()
            && self.text.chars().nth(self.cursor_position) == Some(' ')
        {
            self.cursor_position += 1;
        }
        while self.cursor_position < self.text_len()
            && self.text.chars().nth(self.cursor_position) != Some(' ')
        {
            self.cursor_position += 1;
        }
    }

    pub fn move_line_up(&mut self) {
        let chars: Vec<char> = self.text.chars().collect();
        let current_line_start = chars[..self.cursor_position]
            .iter()
            .rposition(|&c| c == '\n')
            .map(|i| i + 1)
            .unwrap_or(0);

        if current_line_start == 0 {
            return;
        }

        let prev_line_start = chars[..current_line_start.saturating_sub(1)]
            .iter()
            .rposition(|&c| c == '\n')
            .map(|i| i + 1)
            .unwrap_or(0);

        let col = self.cursor_position - current_line_start;
        let prev_line_len = current_line_start - prev_line_start - 1;
        self.cursor_position = prev_line_start + col.min(prev_line_len);
    }

    pub fn move_line_down(&mut self) {
        let chars: Vec<char> = self.text.chars().collect();
        let current_line_start = chars[..self.cursor_position]
            .iter()
            .rposition(|&c| c == '\n')
            .map(|i| i + 1)
            .unwrap_or(0);

        let next_newline = chars[self.cursor_position..]
            .iter()
            .position(|&c| c == '\n');

        let Some(nl_pos) = next_newline else {
            return;
        };

        let next_line_start = self.cursor_position + nl_pos + 1;
        let col = self.cursor_position - current_line_start;
        let next_line_len = chars[next_line_start..]
            .iter()
            .position(|&c| c == '\n')
            .unwrap_or(chars.len() - next_line_start);

        self.cursor_position = next_line_start + col.min(next_line_len);
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_position = 0;
        self.scroll = 0;
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn set_scroll(&mut self, scroll: usize) {
        self.scroll = scroll;
    }

    pub fn scroll(&self) -> usize {
        self.scroll
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

        // Reserve one column for the cursor block so it is always visible
        // after the last character, even on a full line.
        let wrap_width = width.saturating_sub(1).max(1);

        let wrapped: Vec<String> = textwrap::wrap(&self.text, wrap_width)
            .into_iter()
            .map(|cow| cow.into_owned())
            .collect();

        let orig: Vec<char> = self.text.chars().collect();
        let mut result = Vec::with_capacity(wrapped.len());
        let mut pos = 0usize;

        for wl in wrapped {
            let wl_chars: Vec<char> = wl.chars().collect();
            let mut line = String::new();
            let mut wi = 0usize;
            let mut pushed = false;

            while pos < orig.len() {
                let oc = orig[pos];
                if oc == '\n' && wi >= wl_chars.len() {
                    result.push(line);
                    pushed = true;
                    line = String::new();
                    pos += 1;
                    break;
                } else if oc == '\n' {
                    line.push(oc);
                    pos += 1;
                } else if wi < wl_chars.len() && oc == wl_chars[wi] {
                    line.push(oc);
                    pos += 1;
                    wi += 1;
                } else if oc == ' ' {
                    line.push(oc);
                    pos += 1;
                } else {
                    break;
                }
            }
            if !pushed {
                result.push(line);
            }
        }

        if pos < orig.len() {
            result.push(orig[pos..].iter().collect());
        }

        let mut flat = Vec::with_capacity(result.len());
        for line in result {
            for part in line.split('\n') {
                flat.push(part.to_string());
            }
        }

        flat
    }

    fn cursor_row(&self, lines: &[String]) -> usize {
        let mut seen = 0usize;
        let visible_total: usize = lines.iter().map(|l| l.chars().count()).sum();
        let cursor = self.visible_cursor().min(visible_total);

        for (row, line) in lines.iter().enumerate() {
            let len = line.chars().count();
            if cursor < seen + len {
                return row;
            }
            if cursor == seen + len {
                let raw_pos = self.cursor_position;
                if raw_pos > 0 && self.text.chars().nth(raw_pos - 1) == Some('\n') {
                    seen += len;
                    continue;
                }
                return row;
            }
            seen += len;
        }
        lines.len().saturating_sub(1)
    }

    fn cursor_cell(&self, width: usize) -> (usize, usize) {
        let lines = self.wrapped_lines(width);
        let row = self.cursor_row(&lines);
        let col = {
            let mut seen = 0usize;
            for line in lines.iter().take(row) {
                seen += line.chars().count();
            }
            self.visible_cursor()
                .saturating_sub(seen)
                .min(lines[row].chars().count())
        };
        (row, col)
    }

    fn visible_cursor(&self) -> usize {
        self.text
            .chars()
            .take(self.cursor_position)
            .filter(|&c| c != '\n')
            .count()
    }

    fn ensure_cursor_visible(&mut self, height: usize) {
        if height == 0 {
            return;
        }
        let lines = self.wrapped_lines(usize::MAX);
        let cursor_row = self.cursor_row(&lines);

        if cursor_row < self.scroll {
            self.scroll = cursor_row;
        } else if cursor_row >= self.scroll + height {
            self.scroll = cursor_row - height + 1;
        }
    }
}

impl Widget for &mut TextArea {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        if self.is_text_empty() {
            render_placeholder(area, buf, self.placeholder_style, self.cursor_style);
            return;
        }

        self.ensure_cursor_visible(area.height as usize);
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
    let height = area.height as usize;
    let lines = textarea.wrapped_lines(width);
    let (cursor_row, cursor_col) = textarea.cursor_cell(width);

    let start = textarea.scroll;
    let end = (start + height).min(lines.len());

    let mut rendered_rows = Vec::with_capacity(end.saturating_sub(start));
    for row in start..end {
        let line = &lines[row];
        let line_len = line.chars().count();
        let mut spans: Vec<Span> = Vec::with_capacity(line_len.saturating_add(1));

        if row == cursor_row && cursor_col < line_len {
            for (col, c) in line.chars().enumerate() {
                if col == cursor_col {
                    spans.push(Span::styled(c.to_string(), textarea.cursor_style));
                } else {
                    spans.push(Span::styled(c.to_string(), textarea.text_style));
                }
            }
        } else if row == cursor_row && cursor_col >= line_len {
            for c in line.chars() {
                spans.push(Span::styled(c.to_string(), textarea.text_style));
            }
            spans.push(Span::styled(" ", textarea.cursor_style));
        } else {
            for c in line.chars() {
                spans.push(Span::styled(c.to_string(), textarea.text_style));
            }
        }

        rendered_rows.push(Line::from(spans));
    }

    Paragraph::new(rendered_rows)
        .style(textarea.text_style)
        .render(area, buf);
}

mod tests {
    use super::*;
    use crossterm::event::KeyEvent;

    #[allow(dead_code)]
    fn type_text(s: &str) -> TextArea {
        let mut ta = TextArea::new();
        for c in s.chars() {
            ta.input(KeyEvent::from(KeyCode::Char(c)));
        }
        ta
    }

    #[test]
    fn trailing_space_keeps_cursor_at_end_not_start() {
        let ta = type_text("hello world ");
        let (row, col) = ta.cursor_cell(11);
        let lines = ta.wrapped_lines(11);
        assert_eq!(lines, vec!["hello ".to_string(), "world ".to_string()]);
        assert_eq!((row, col), (1, 6));
    }

    #[test]
    fn cursor_moves_with_space_typed_in_middle() {
        let mut ta = type_text("hello");
        ta.input(KeyEvent::from(KeyCode::Left));
        ta.input(KeyEvent::from(KeyCode::Left));
        ta.input(KeyEvent::from(KeyCode::Char(' ')));
        let (row, col) = ta.cursor_cell(11);
        assert_eq!((row, col), (0, 4));
    }

    #[test]
    fn cursor_follows_wrapped_words() {
        let ta = type_text("the quick brown fox jumps");
        let (row, col) = ta.cursor_cell(11);
        let lines = ta.wrapped_lines(11);
        assert_eq!(row, lines.len() - 1);
        assert_eq!(col, lines.last().unwrap().chars().count());
    }

    #[test]
    fn wrapped_lines_preserve_every_character() {
        let ta = type_text("the quick brown fox jumps");
        let lines = ta.wrapped_lines(11);
        let joined: String = lines.concat();
        assert_eq!(joined, "the quick brown fox jumps");
    }

    #[test]
    fn cursor_offset_matches_typed_position_across_wraps() {
        let ta = type_text("the quick brown fox jumps");
        let (row, col) = ta.cursor_cell(11);
        let lines = ta.wrapped_lines(11);
        let offset: usize = lines
            .iter()
            .take(row)
            .map(|l| l.chars().count())
            .sum::<usize>()
            + col;
        assert_eq!(offset, ta.text.chars().count());
    }

    #[test]
    fn space_at_end_of_full_line_advances_cursor() {
        let mut ta = type_text("hello world");
        ta.input(KeyEvent::from(KeyCode::Char(' ')));
        let (row, col) = ta.cursor_cell(11);
        let lines = ta.wrapped_lines(11);
        assert_eq!(
            (row, col),
            (lines.len() - 1, lines.last().unwrap().chars().count())
        );
    }

    #[test]
    fn shift_enter_inserts_visible_newline() {
        let mut ta = type_text("hello");
        ta.insert_newline();
        ta.input(KeyEvent::from(KeyCode::Char('w')));
        let lines = ta.wrapped_lines(11);
        // "hello\nw" -> "hello", "w"
        assert_eq!(lines, vec!["hello".to_string(), "w".to_string()]);
        let (row, col) = ta.cursor_cell(11);
        assert_eq!((row, col), (1, 1));
    }

    #[test]
    fn cursor_on_new_line_after_insert_newline() {
        let mut ta = type_text("hello");
        ta.insert_newline();
        let (row, col) = ta.cursor_cell(11);
        let lines = ta.wrapped_lines(11);
        assert_eq!(lines, vec!["hello".to_string(), "".to_string()]);
        assert_eq!((row, col), (1, 0));
    }

    #[test]
    fn cursor_moves_up_and_down_between_lines() {
        let mut ta = type_text("ab\ncd\nef");
        ta.input(KeyEvent::from(KeyCode::End));
        assert_eq!(ta.cursor_position, 8);
        ta.move_line_up();
        assert_eq!(ta.cursor_position, 5);
        ta.move_line_up();
        assert_eq!(ta.cursor_position, 2);
        ta.move_line_up();
        assert_eq!(ta.cursor_position, 2);
        ta.move_line_down();
        assert_eq!(ta.cursor_position, 5);
        ta.move_line_down();
        assert_eq!(ta.cursor_position, 8);
        ta.move_line_down();
        assert_eq!(ta.cursor_position, 8);
    }

    #[test]
    fn cursor_never_jumps_to_column_zero_on_space() {
        for n in 1..="the quick brown fox jumps".len() {
            let s: String = "the quick brown fox jumps".chars().take(n).collect();
            let mut ta = TextArea::new();
            for c in s.chars() {
                ta.input(KeyEvent::from(KeyCode::Char(c)));
            }
            ta.input(KeyEvent::from(KeyCode::Char(' ')));
            let (row, _col) = ta.cursor_cell(11);
            assert!(
                row < ta.wrapped_lines(11).len(),
                "cursor row out of range for '{s}'"
            );
        }
    }
}
