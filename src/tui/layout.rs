use crate::app::Message;
use crate::app::Role;
use crate::tui::textarea::TextArea;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget, Widget,
    },
};

pub const ACCENT: Color = Color::Rgb(86, 156, 255);
pub const ACCENT_SOFT: Color = Color::Rgb(120, 170, 255);
pub const FG: Color = Color::Rgb(235, 238, 245);
pub const BG: Color = Color::Rgb(13, 17, 23);
pub const USER: Color = Color::Rgb(86, 156, 255);
pub const ASSISTANT: Color = Color::Rgb(120, 200, 160);

fn build_chat_lines(messages: &[Message]) -> Vec<Line<'static>> {
    let mut lines: Vec<Line> = Vec::new();
    for msg in messages {
        let (label, color) = match msg.role {
            Role::User => ("You", USER),
            Role::Assistant => ("stow", ASSISTANT),
        };

        lines.push(Line::from(vec![Span::styled(
            format!("{label} "),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )]));

        for content_line in msg.content.lines() {
            lines.push(Line::from(Span::styled(
                content_line.to_string(),
                Style::default().fg(FG),
            )));
        }
        lines.push(Line::from(""));
    }
    lines
}

pub fn render_chat_area(frame: &mut Frame, area: Rect, messages: &[Message], scroll: i32) {
    let block = Block::default()
        .borders(Borders::NONE)
        .padding(Padding::horizontal(2));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = build_chat_lines(messages);
    let content_height = lines.len() as u16;
    let view_height = inner.height;
    let max_scroll = content_height.saturating_sub(view_height) as i32;
    let scroll = scroll.min(max_scroll);

    let visible: Vec<Line> = lines
        .into_iter()
        .enumerate()
        .skip(scroll as usize)
        .take(view_height as usize)
        .map(|(_, line)| line)
        .collect();

    Paragraph::new(visible)
        .alignment(Alignment::Left)
        .render(inner, frame.buffer_mut());

    if content_height > view_height {
        let mut state = ScrollbarState::new(content_height as usize).position(scroll as usize);
        StatefulWidget::render(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            inner,
            frame.buffer_mut(),
            &mut state,
        );
    }
}

pub fn render_input(frame: &mut Frame, area: Rect, chat_input: &mut TextArea) {
    let prompt = Span::styled(
        "❯ ",
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
    );

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::Rgb(40, 46, 58)))
        .padding(Padding::horizontal(2));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let prompt_width = 3u16;
    let text_area = Rect {
        x: inner.x + prompt_width,
        y: inner.y,
        width: inner.width.saturating_sub(prompt_width),
        height: inner.height,
    };

    Paragraph::new(Line::from(vec![prompt])).render(
        Rect {
            x: inner.x,
            y: inner.y,
            width: prompt_width,
            height: 1,
        },
        frame.buffer_mut(),
    );

    frame.render_widget(chat_input, text_area);
}
