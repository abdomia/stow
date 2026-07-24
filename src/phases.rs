use crate::app::AppFlow;
use crate::frame_to_text;
use crate::intro_anim::LogoAnim;
use crate::tui::layout::{render_chat_area, render_input};
use crate::{ACCENT, ACCENT_SOFT};

use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::{Frame, layout::Rect, widgets::Paragraph, widgets::Widget};

use std::time::{Duration, Instant};

#[derive(Clone)]
pub enum AppPhase {
    Splash(Instant),
    Reveal { start: Instant, pixels: Vec<u8> },
    Normal,
}

pub fn updated_phase(phase: AppPhase, anim: &LogoAnim) -> AppPhase {
    match phase {
        AppPhase::Splash(time) if time.elapsed() >= anim.total_duration() => {
            let px = anim.last_frame().to_vec();
            AppPhase::Reveal {
                start: Instant::now(),
                pixels: px,
            }
        }
        AppPhase::Reveal { start, .. } if start.elapsed() >= Duration::from_millis(700) => {
            AppPhase::Normal
        }
        other => other,
    }
}

impl AppPhase {
    pub fn display_phase(
        &self,
        frame: &mut Frame,
        area: &Rect,
        w: u16,
        h: u16,
        term_h: u16,
        app_flow: &mut AppFlow,
        anim: &LogoAnim,
    ) {
        match self {
            AppPhase::Splash(start) => {
                let x = area.x + area.width.saturating_sub(w) / 2;
                let y = area.y + area.height.saturating_sub(term_h) / 2;
                if let Some(pixels) = anim.frame_at(start.elapsed()) {
                    let text = frame_to_text(pixels, w, h);
                    Paragraph::new(text).render(
                        Rect {
                            x,
                            y,
                            width: w,
                            height: term_h,
                        },
                        frame.buffer_mut(),
                    );
                }
            }
            AppPhase::Reveal { start, pixels } => {
                let elapsed = start.elapsed();
                let duration = Duration::from_millis(700);
                let t = (elapsed.as_secs_f32() / duration.as_secs_f32()).min(1.0);
                let eased = ease_out_cubic(t);

                let start_y = area.y + (area.height - term_h) / 2;
                let move_up = (area.height as f32 * 0.25) * eased;
                let gif_y = (start_y as f32 - move_up).max(0.0) as u16;

                let gif_area = Rect {
                    x: area.x + (area.width - w) / 2,
                    y: gif_y,
                    width: w,
                    height: term_h,
                };
                let text = frame_to_text(pixels, w, h);
                Paragraph::new(text).render(gif_area, frame.buffer_mut());

                let text_y = gif_y + term_h + 2;
                if text_y + 1 < area.bottom() {
                    let stow = Paragraph::new(Line::from(Span::styled(
                        "Stow",
                        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
                    )))
                    .alignment(Alignment::Center);
                    stow.render(
                        Rect {
                            x: area.x,
                            y: text_y,
                            width: area.width,
                            height: 1,
                        },
                        frame.buffer_mut(),
                    );
                }
            }
            AppPhase::Normal => {
                let has_msgs = !app_flow.messages.is_empty();

                let (_header_chunk, msg_chunk, input_chunk) = if has_msgs {
                    let c = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Min(0), Constraint::Length(8)])
                        .split(*area);
                    (None, c[0], c[1])
                } else {
                    let start_y = area.y + (area.height - term_h) / 2;
                    let gif_y = (start_y as f32 - area.height as f32 * 0.25).max(0.0) as u16;
                    let text_y = gif_y + term_h + 2;
                    let header_end = text_y + 2;

                    let c = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(header_end.saturating_sub(area.y)),
                            Constraint::Length(8),
                            Constraint::Min(0),
                        ])
                        .split(*area);

                    let text = frame_to_text(anim.last_frame(), w, h);
                    Paragraph::new(text).render(
                        Rect {
                            x: area.x + (area.width - w) / 2,
                            y: gif_y,
                            width: w,
                            height: term_h,
                        },
                        frame.buffer_mut(),
                    );

                    if text_y + 1 < area.bottom() {
                        let stow = Paragraph::new(Line::from(Span::styled(
                            "Stow",
                            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
                        )))
                        .alignment(Alignment::Center);
                        stow.render(
                            Rect {
                                x: area.x,
                                y: text_y,
                                width: area.width,
                                height: 1,
                            },
                            frame.buffer_mut(),
                        );

                        let tagline = Paragraph::new(Line::from(Span::styled(
                            "The Coding Agent For You",
                            Style::default().fg(ACCENT_SOFT),
                        )))
                        .alignment(Alignment::Center);
                        tagline.render(
                            Rect {
                                x: area.x,
                                y: text_y + 1,
                                width: area.width,
                                height: 1,
                            },
                            frame.buffer_mut(),
                        );
                    }

                    (Some(c[0]), c[2], c[1])
                };

                app_flow.clamp_scroll(msg_chunk.height);
                render_input(frame, input_chunk, &mut app_flow.prompt_area);
                render_chat_area(frame, msg_chunk, &app_flow.messages, app_flow.chat_scroll);
            }
        }
    }
}

pub fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(6)
}
